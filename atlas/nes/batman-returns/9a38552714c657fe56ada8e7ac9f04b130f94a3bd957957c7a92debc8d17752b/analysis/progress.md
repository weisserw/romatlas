# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Coordinator log

### 2026-11-14 — Round 1, phase 1 (disassembly): BLOCKED before start

- Round: 1. Current phase: disassembly (prompts/disassembly.txt), not started.
- Input checkpoint: none (no baseline, no analysis/state.json content beyond init,
  no maps/baseline.json). Output checkpoint: none.
- Leads consumed/produced: none.
- Blocker (environment, not toolkit): every shell command in the coordinator
  session fails before execution with
  `apply-seccomp: write /proc/self/setgroups (nested userns is capability-restricted;
  caller must provide CAP_SYS_ADMIN): Permission denied` — even `echo hi`.
  The workflow requires `python3 run.py inspect|baseline|serve|checkpoint`
  (workflow.md steps 1-7), so no phase worker can run inspection, discovery,
  emulation, map save/export or verification. Phase workers share the same
  sandbox, so launching them would not get past this.
- No ROM classification percentage exists yet (0 classified; baseline not run).
- Nothing was modified other than this log. input/game.nes untouched.
- Resume recipe: once shell execution works in the runner sandbox, start
  Round 1 disassembly from workflow.md step 1 (`python3 run.py inspect`, then
  `python3 run.py baseline` once). Do not re-initialize the workspace.
- archive-options.json not written: no final analysis/commentary documents
  exist yet; it belongs to terminal handoff.

### 2026-11-19 — Round 1, phase 1 (disassembly): resumed

- Prior seccomp shell blocker no longer reproduces (`python3 run.py inspect` succeeds;
  configured binary /root/romatlas/target/release/romatlas present).
- Launching Round 1 disassembly worker (prompts/disassembly.txt). Baseline absent,
  worker establishes it once. Free disk at start: ~4.8 GB.
- Phase handoff file: analysis/handoffs/round-01-disassembly.md (worker writes).

## Round 1 — disassembly pass (2026-11-19)

ROM classification: 86.76% (341171 / 393232 bytes classified; 52061 bytes unknown).

Status: INCOMPLETE (52,061 PRG bytes unknown). Checkpoint `pass-001`: maps/pass-001.json,
generated/pass-001, analysis/pass-001-verification.json — ok=true, byte_identical=true
(map sha256 97e32861...). Baseline established once: maps/baseline.json, generated/baseline (verified).

Game identified: Batman Returns (Konami 1993), MMC3, PRG mode 0 in all observed contexts.

Per-bank counts (code / data+image / unknown bytes):

| Area | Code | Data | Unknown |
| --- | ---: | ---: | ---: |
| Header | 0 | 16 | 0 |
| PRG 0 | 3443 | 292 | 4457 |
| PRG 1 | 0 | 8192 | 0 |
| PRG 2 | 0 | 8192 | 0 |
| PRG 3 | 0 | 8192 | 0 |
| PRG 4 | 6196 | 154 | 1842 |
| PRG 5 | 4508 | 505 | 3179 |
| PRG 6 | 1663 | 31 | 6498 |
| PRG 7 | 820 | 90 | 7282 |
| PRG 8 | 0 | 8192 | 0 |
| PRG 9 | 195 | 7997 | 0 |
| PRG 10 | 3117 | 190 | 4885 |
| PRG 11 | 3907 | 207 | 4078 |
| PRG 12 | 544 | 19 | 7629 |
| PRG 13 | 204 | 731 | 7257 |
| PRG 14 | 6689 | 75 | 1428 |
| PRG 15 | 3833 | 833 | 3526 |
| CHR (Image) | 0 | 262144 | 0 |

Code provenance: 21,584 bytes Observed (instruction fetches; natural boot/title/intro/stage 1-1,
plus artificial $3E stage tour), 13,535 bytes Static (recursive descent, confidence 0.85).
Annotation coverage: 1122 regions (1055 Code, 66 Data, 1 Image), all with evidence text; labels
Reset/NMI/IRQ/JumpEngine/SoundJumpEngine/SetPrgPairFromStack/INesHeader/ChrRom/CpuVectors/KonamiIdBlock.
Execution coverage (separate metric): 13,041 unique contextual instructions observed.

Scenarios: boot -> title -> menu -> intro -> stage 1-1 (natural); RAM $003E stage tour 0x00-0x15
(all 22 stage values incl. 6-2 part 2; artificial). Gains: 8,510 -> 13,041 unique instructions; last
8 stage values added <300 combined, so short-stage play stopped yielding substantial gains. Bosses
and ending not reached (L006). Details: analysis/scenarios.json, analysis/gameplay.md.

Leads: consumed none (none existed); produced L001-L006 (analysis/leads.json). Investigation:
analysis/investigation.md. Handoff: analysis/handoffs/round-01-disassembly.md.

Toolkit: 3 findings in analysis/toolkit-issues.md — TK-001 confirmed bug (NES emulator.input
ignores documented button/pressed form), TK-002 missing feature/perf (tracing ~12x overhead),
TK-003 missing feature (no data-read aggregate). None blocking.

### Coordinator — Round 1 disassembly complete (2026-11-19)

- Output checkpoint: pass-001 (verified ok, byte-identical). Percentage printed to user:
  ROM classification: 86.76% (341171 / 393232 bytes classified; 52061 bytes unknown).
- Leads produced: L001–L006 (none consumed). Handoff: analysis/handoffs/round-01-disassembly.md.
- Next: Round 1 commentary (prompts/commentary.txt), handoff analysis/handoffs/round-01-commentary.md.

## Round 1 — commentary pass (2026-11-19)

- Prepared commentary/ from pass-001 (verified). Rendered commented source via
  analysis/tools-commentary/ (annotation DSL in commentary/ann/). All 636 routines named and
  commented, ~250 RAM symbols, jump tables as .word label lists, unknown-range notes.
- Verification: commentary verify ok, byte_identical. Reading copy commentary/reading-after.asm.
- Readability gate INCOMPLETE: in-routine flow comments/branch names for large routines, sound/
  state constants and some scratch RAM remain (exact ranges in
  analysis/handoffs/round-01-commentary.md). Classification unchanged (86.76%).

### Coordinator — Round 1 commentary part 1 (2026-11-19): INCOMPLETE, resuming same phase

- Commentary prepared from pass-001; commentary.py verify byte-identical; reading copy
  commentary/reading-after.asm. 636 routines named/commented, review.json all reviewed.
- Gate not met: 19 large routines lack internal comments/branch names, generic boss/
  vehicle names, raw sound/object constants, ~45 scratch RAM unnamed, 213 placeholder
  table names. Exact ranges: analysis/handoffs/round-01-commentary.md.
- Resuming commentary (part 2) before redirect, per goal.md gate.

### Coordinator — Round 1 commentary part 2 (2026-11-19): COMPLETE (readability gate)

- All handoff gaps closed; fresh commentary.py verify byte-identical; reading copy refreshed;
  review.json 1355/1355. Classification unchanged 86.76%. Deferred to final acceptance:
  ~1300 mechanical _Skip/_Loop branch labels outside the 19 large routines; stage 3-3 boss.
- Handoff: analysis/handoffs/round-01-commentary.md (Part 1 + Part 2).
- Next: Round 1 redirect (prompts/redirect.txt), records under analysis/redirect/r01-*/,
  handoff analysis/handoffs/round-01-redirect.md.

## Round 1 — redirect (attempt r01-a, 2026-11-19)

- Input checkpoint pass-001 (unchanged). ROM classification unchanged: 86.76% (341171 / 393232; 52061 unknown).
- Method: improved bank-aware static descent (pair-preserving JSR, MapBanks/BankPop stack, JumpEngine and
  local jmp (zp) table parsing) in analysis/redirect/r01-a/tools; bounded artificial runtime checks.
- Findings: ObjHandlerTable $DE19 (62 entries, pair 6/7 + fixed 15, $1F also 10/11) ; VehObjHandlerTable (45);
  bank-10 intro/cutscene tables; stage-event dispatch (bank 5 / bank 10, guarded 4/5 path); bank-11 boss
  state table $B554; Mode6/Mode7 (continue / ending) dispatch; data proposals for bank 0 sound data, banks 12/13,
  bank 11 text/tables, bank 14 map lists, quarter-wave table; one map correction (bank-10 region 1066 tail is code).
- Static estimate: 27,269 new code bytes (batch ALL) + ~18.8 KB data proposals; observed checks: stage 3-3 boss
  (416 fetches, all predicted) and ending via GameMode=7 (133 fetches, all predicted; credits rendered).
- Leads: ready L001, L003, L004, L007-L016; L002/L005 subsumed; L006 open (natural ending not played).
- Handoff: analysis/handoffs/round-01-redirect.md. Another disassembly round is warranted.

### Coordinator — Round 1 redirect complete (2026-11-19)

- Attempt dir analysis/redirect/r01-a/; handoff analysis/handoffs/round-01-redirect.md.
- Ready leads: L001, L003, L004, L007–L016 (L002/L005 subsumed). ~27.3 KB code + ~18.8 KB
  data proposed (static; two artificial runtime checks consistent). Ending/credits shown via
  forced game mode 7 (artificial); natural ending (L006) still open.
- Decision: new ready leads → Round 2 disassembly (input checkpoint pass-001).

## Round 2 — disassembly pass (2026-11-19)

ROM classification: 100.00% (393223 / 393232 bytes classified; 9 bytes unknown).

Status: INCOMPLETE (9 PRG bytes unknown; whole-ROM contract not met). Input checkpoint pass-001; output
checkpoint `pass-002`: maps/pass-002.json, generated/pass-002, analysis/pass-002-verification.json —
ok=true, byte_identical=true (map sha256 794b0e91...). Store analysis/state.json restored (map.load) and
saved; maps/current.json exported; map.validate ok. Baseline not re-run. Existing classifications preserved
(only edits: region 1066 shrunk by L010; two Round-1 truncated inline tables in bank 7 extended).

Per-bank counts (code / data+image / unknown bytes):

| Area | Code | Data | Unknown |
| --- | ---: | ---: | ---: |
| Header | 0 | 16 | 0 |
| PRG 0 | 3443 | 4749 | 0 |
| PRG 1 | 0 | 8192 | 0 |
| PRG 2 | 0 | 8192 | 0 |
| PRG 3 | 0 | 8192 | 0 |
| PRG 4 | 7171 | 1020 | 1 |
| PRG 5 | 7123 | 1067 | 2 |
| PRG 6 | 7486 | 706 | 0 |
| PRG 7 | 7191 | 1001 | 0 |
| PRG 8 | 0 | 8192 | 0 |
| PRG 9 | 195 | 7997 | 0 |
| PRG 10 | 7051 | 1140 | 1 |
| PRG 11 | 7036 | 1155 | 1 |
| PRG 12 | 966 | 7226 | 0 |
| PRG 13 | 284 | 7908 | 0 |
| PRG 14 | 7097 | 1092 | 3 |
| PRG 15 | 6616 | 1575 | 1 |
| CHR (Image) | 0 | 262144 | 0 |

Totals: Code 61,659 B (21,584 Observed + 40,075 Static), Data 69,420 B, Image 262,144 B; 1902 regions
(was 1122). Gain this pass: +52,052 B (26,540 code, 25,512 data net).

Batches (all Static provenance, scripts and region lists in analysis/tools-round2/):
1. L010 correction (l010.py): 14 B code, region 1066 -> 120 B data.
2. r01-a static descent rerun on the refreshed map (attrib.py/build_code.py; batch ALL = L001+L007+L008+L009+L011+L012
   roots; 0 illegal, 0 overlaps, 0 data conflicts): 25,619 B code + 1,918 B jump tables (incl. manual $DE19, $910E, $CE4C,
   $CE6C/$CE81) -> batch1-regions.json.
3. JSR fall-through holes (batch2.py with desc2.py: fall-through also after jmp(zp)-dispatcher callees and BankPopAndRestore):
   400 B code + 3 small tables -> batch2-regions.json.
4. Data leads L004/L013/L014/L015/L016/L003 (build_data.py): 19,811 B data -> data-regions.json.
5. Truncated Round-1 inline tables at bank-7 $B649/$B763 extended (tblext.py) + handler $B70D (batch3.py): 65 B code,
   36 B data.
6. Residual holes (gapscan.py, cat_gaps.py, coherent.py, build4.py): consumer-read tables (A categories) as data,
   coherent unreferenced routines as Static code (confidence 0.7, reachability not claimed), visibly non-instruction
   holes / bank-end $EA trailers / bank-14 ID byte as data: 442 B code + 3,737 B data -> batch4-regions.json.

Verification: every observed instruction start from Round-1 traces (9,788) and r01-a probes (549) and the new Round-2
probes (5,285) is an instruction start of a Code region (0 contradictions; analysis/tools-round2/obscheck.py,
covcheck.py; analysis/evidence-round2/probe-coverage-check.json). 428 Round-2 probe fetches fall in Round-2 Static regions.

Gameplay: final stage $15 boss reached (artificial stage write); natural defeat not achieved in bounded probes; forced
defeat flags led to StageState 4 -> GameMode 7 and fetched the ending trigger $B7B9 (artificial). analysis/scenarios.json.

Remaining unknown (9 B): 0x08743, 0x0B1BE, 0x0B7A2, 0x14CCA, 0x17BC6, 0x1C1E2-0x1C1E4, 0x1FD5A — isolated unreferenced
RTS / JMP $FBCC after terminal instructions (lead L017).

Leads: verified L001, L003, L004, L007, L008, L009, L010, L011, L012, L013, L014, L015, L016; superseded L002, L005;
L006 updated (still open); produced L017, L018. None rejected or blocked.

Toolkit: analysis/toolkit-issues.md — new TK-005 (minor doc mismatch: map.export include_sym=false reply lacks path/bytes);
TK-001..TK-004 unchanged. None blocking.
Handoff: analysis/handoffs/round-02-disassembly.md.

### Coordinator — Round 2 disassembly complete (2026-11-19)

- Input pass-001 → output checkpoint pass-002 (verified ok, byte-identical).
- Printed to user: ROM classification: 100.00% (393223 / 393232 bytes classified; 9 bytes unknown).
- Leads verified: L001, L003, L004, L007–L016 (L010 first). L002/L005 superseded.
  New: L017 (9 isolated unknown single instructions), L018 (442 B no-caller routines, conf 0.7).
- Final boss reached via stage write; defeat forced → ending ($B7B9) executed (artificial). L006 open.
- Next: Round 2 commentary (carry forward round-1 commentary; preserve under analysis/round-1/ per prompt).

## Round 2 commentary (2026-11-19)
- Readability gate complete on pass-002; fresh verify byte-identical; reading copy refreshed. Round-1 commentary preserved in analysis/round-1/. Handoff: analysis/handoffs/round-02-commentary.md.

### Coordinator — Round 2 commentary complete (2026-11-19)

- Round-1 commentary preserved at analysis/round-1/; fresh pass-002 commentary baseline;
  carry-forward by physical range. Gate COMPLETE; verify byte-identical; reading copy
  refreshed; review.json 1909/1909. Handoff analysis/handoffs/round-02-commentary.md.
- Next: Round 2 redirect (attempt dir analysis/redirect/r02-a/), focus L017 (9 bytes), L006,
  L018, open semantic questions.

## Round 2 redirect (attempt r02-a, 2026-11-19)
- Input/output checkpoint pass-002 (no map change); classification 100.00% (393223 / 393232; 9 B unknown).
- L017 reopened with combined evidence (bank-aware reference audit, data-consumer audit, JMP->RTS idiom census,
  non-fill, ending run without fetches) -> READY proposal: classify the 9 B as Code/Static dead tails (no reachability).
- L019 (new, READY, commentary only): ObjT33 runs in pair 6/7 ($8754 = ObjT03_Spawn, $BE86 = ObjT32_Handler_St00_St01_Skip;
  confirmed by artificial stage-$11 wave trigger), $32/$33 spawned by stage $11 wave script, 9-byte wave record format,
  StageEventIndex 3 cleared in StageLoadGraphics before StageEventDispatch, boss-loop pair census.
- L006 verified (artificial assist): final boss defeated by real player hits ($0145=8, health refresh) -> GameMode 7 ending;
  snapshot traces/round2/final_boss_defeated.snap. L018 blocked (no caller exists; unchanged Static 0.7).
- Handoff: analysis/handoffs/round-02-redirect.md; evidence analysis/redirect/r02-a/.

### Coordinator — Round 2 redirect complete (2026-11-19)

- Attempt analysis/redirect/r02-a/; handoff analysis/handoffs/round-02-redirect.md.
- Ready: L017 (9 B, proposed Code/Static unreachable tails, conf <=0.6, no reachability claim),
  L019 (commentary-only facts). L018 blocked (no refs; stays conf 0.7). L006: ending reached
  with assisted fight (boss HP patched, health refilled) — artificial assistance recorded.
- Decision: Round 3 disassembly to review/apply L017 (input pass-002).

## Round 3 disassembly (pass-003, 2026-11-19)

ROM classification: 100.00% (393229 / 393232 bytes classified; 3 bytes unknown).

- Input checkpoint pass-002; state restored from analysis/state.json (all existing classifications preserved);
  baseline not re-run. Output checkpoint pass-003: map validate ok, generate + verify ok, byte_identical true
  (analysis/pass-003-verification.json, maps/pass-003.json, generated/pass-003).
- Per bank (classified / unknown): header 16/0; PRG banks 0-9 8192/0 each; bank 10 8191/1 (0x14CCA);
  bank 11 8191/1 (0x17BC6); banks 12-14 8192/0 each; bank 15 8191/1 (0x1FD5A); CHR 262144/0.
- L017 review (evidence-policy.md): split acceptance. Applied 6 B as Code / Static / confidence 0.6, no label,
  reachability not claimed (region ids 1903-1906): 0x08743 (b4 $8733), 0x0B1BE (b5 $B1AE), 0x0B7A2 (b5 $B792) -
  each strictly inside a classified branch span of its routine (analysis/tools-round3/l017-span.json);
  0x1C1E2-0x1C1E4 (b14 $C1D2 JMP $FBCC = reset entry, exact gap between fixed-bank Code). Rejected (kept unknown):
  0x14CCA, 0x17BC6, 0x1FD5A - lone unreferenced RTS at routine boundaries without enclosing structure, twin or consumer
  (policy: unreferenced return opcode alone insufficient). Tools: analysis/tools-round3/.
- No gameplay this pass (broadly explored already; no concrete gain). L018 blocked (unchanged), L019 commentary-only.
- Toolkit: no new toolkit issues found; existing TK-001..TK-005 unchanged (analysis/toolkit-issues.md).
- Status: incomplete (3 B unknown terminal residual). Handoff: analysis/handoffs/round-03-disassembly.md.

### Coordinator — Round 3 disassembly complete (2026-11-19)

- Input pass-002 → output pass-003 (verified ok, byte-identical).
- Printed to user: ROM classification: 100.00% (393229 / 393232 bytes classified; 3 bytes unknown).
- L017 partially verified (6 B: 0x08743, 0x0B1BE, 0x0B7A2, 0x1C1E2–4); 3 B rejected/kept unknown
  (0x14CCA, 0x17BC6, 0x1FD5A — lone unreferenced RTS, no discriminating evidence).
- Next: Round 3 commentary (carry pass-002 commentary forward to pass-003; apply L019 facts).

### Coordinator — Round 3 commentary complete (2026-11-19)

- Round-2 commentary preserved at analysis/round-2/; fresh pass-003 baseline, carry-forward;
  gate COMPLETE; verify byte-identical; reading copy refreshed; L019 applied & verified.
  Handoff analysis/handoffs/round-03-commentary.md.
- Next: Round 3 redirect (analysis/redirect/r03-a/), 3 residual bytes + terminal review.

## Round 3 redirect (attempt r03-a, 2026-11-19)

- Input checkpoint pass-003; no map/state/source change. Classification 100.00% (393229 / 393232 B; 3 B unknown:
  0x14CCA b10 $8CBA, 0x17BC6 b11 $BBB6, 0x1FD5A b15 $FD4A).
- Re-examined the 3 L017 residual RTS bytes with new methods: fall-through (predecessors unconditional),
  following-record base (Code on both sides), anti-tamper checksum windows (none cover), RTS-trick/computed
  dispatch (none), relocation-tolerant twin search (no copy >3 instructions), runtime access watch (0x14CCA:
  0 accesses in a natural 6786-frame boot->intro->stage run with positive control; 0x1FD5A: only RTS dummy
  reads of PC+1, 297/297 cycle-paired), precedent consistency. All negative / non-discriminating.
- Result: no ready lead; terminal incomplete recommended with rejection review
  (analysis/handoffs/round-03-redirect.md). leads.json: L017 attempt + pass record; scenarios.json: password
  milestone attempted (unconfirmed); toolkit-issues.md: TK-003 addendum (dummy reads reported as Read).
- Evidence: analysis/redirect/r03-a/evidence/ (residual-static.json, twins.json, readwatch-14CCA-intro.json,
  readwatch-1FD5A-password.json, dummyread-1FD5A.json). No server running.

### Coordinator — Round 3 redirect complete; TERMINAL INCOMPLETE handoff (2026-11-19)

- Attempt analysis/redirect/r03-a/; handoff analysis/handoffs/round-03-redirect.md. No ready leads.
- Coordinator inspected the rejection review and decisive artifacts (r03-a/evidence/residual-static.json,
  twins.json, readwatch-*.json, dummyread-1FD5A.json; r02-a l017 refs/reads; tools-round3/l017-span.json):
  sufficiency judgement consistent with accepted per-instance standards; no corrected inference or
  untried discriminating test identified. Accepted as terminal incomplete; not cycling.
- Final: ROM classification: 100.00% (393229 / 393232 bytes classified; 3 bytes unknown).
  Residual: 0x14CCA, 0x17BC6, 0x1FD5A. Checkpoint pass-003 (byte-identical). Map/state validate.
- Commentary: gate complete; fresh commentary.py verify byte-identical (re-run by coordinator);
  editorial_status: 1909/1909 regions, 2117/2117 units reviewed; final editorial audit pending
  (reserved for final acceptance/archive).
- archive-options.json written: selects all phase handoffs + final-terminal.md; omits analysis/round-1|2/
  commentary/*.md (toolkit boilerplate copies and superseded progress/uncertainties snapshots whose
  content is carried forward into commentary/progress.md and commentary/uncertainties.md; files remain
  in the workspace). All analysis/ and commentary/ Markdown accounted for. No archive executed.
- Terminal handoff: analysis/handoffs/final-terminal.md. Workflow termination ≠ ROM completion.

## Coordinator — user-authorized residual fallback (batch-only exception, 2026-11-19)

- Explicit user instruction (batch runner): when <= 10 physical ROM bytes remain unclassified after the terminal
  incomplete handoff, mark them as generic Data with UNKNOWN meaning, Manual provenance. This is a user-directed
  fallback, NOT an observed or statically proven data role; it overrides the further-structural-proof rule for these
  residual bytes only. No return to open-ended redirect research for them.
- Coordinator verified the exact complement of maps/current.json (== maps/pass-003.json; state loads):
  3 bytes unknown — phys 0x14CCA (85194, b10 $8CBA), 0x17BC6 (97222, b11 $BBB6), 0x1FD5A (130394, b15 $FD4A).
  Matches the user-validated list. 3 <= 10, exception applies.
- Prior ambiguity (preserved): lone unreferenced RTS-valued bytes at routine boundaries; code-vs-data materially
  ambiguous; L017 residual rejection review analysis/handoffs/round-03-redirect.md; evidence r02-a / r03-a.
- Plan: phase worker A (fallback annotation, save/export/validate, checkpoint pass-004);
  phase worker B (commentary carry-forward to pass-004 + final review + strict editorial audit); then
  archive-options.json update. No archive/delete/publish/toolkit edits.

## Round 4 fallback annotation (pass-004)

- User-authorized residual fallback (batch-only exception, <=10 bytes, authorization 2026-11-19). Complement verified
  = exactly 3 bytes before editing: 0x14CCA (b10 $8CBA), 0x17BC6 (b11 $BBB6), 0x1FD5A (b15 $FD4A).
- Annotated as 1-byte Data, Manual, confidence 0.3, no label, views [] -> regions 1907, 1908, 1909. Evidence/comment
  state user-directed fallback, meaning unknown, NOT observed/statically proven, code-vs-data ambiguous
  (analysis/handoffs/round-03-redirect.md), reachability not claimed. Script/records: analysis/tools-fallback/.
- validate ok; state saved; maps/current.json exported; all 1906 pass-003 regions unchanged; analysis.unknown total 0.
- Checkpoint pass-004: ok, byte_identical true (analysis/pass-004-verification.json).
- ROM classification: 100.00% (393232 / 393232 bytes classified); 3 bytes are Manual user-directed fallback Data
  (unknown meaning), NOT evidence-backed.
- Commentary todo: comments for regions 1907-1909 (unknown meaning + user fallback). Handoff:
  analysis/handoffs/round-04-fallback.md.

## Round 4 commentary + final audit

- Round-3 commentary preserved (analysis/round-3/, diff -r identical); fresh prepare on pass-004; names/comments/review
  carried by physical range; regions 1907-1909 and neighbours re-reviewed.
- Fallback bytes documented in source as user-directed manual fallback data (meaning unknown, $60 = RTS, code-vs-data
  ambiguous, no reachability claim). Final editorial pass: 4 misleading names fixed, stale comments/names removed,
  5 tables named, RAM notes regenerated, all units commented.
- `commentary.py audit` PASSED: ok, byte_identical, 1909/1909 regions, 2117/2117 units, editorial_audit true.
  verify byte-identical (game.asm sha256 712a9ad5...); reading copy refreshed.
- Classification: 393232/393232 bytes, 3 of them by manual user-directed fallback (not evidence-backed).
- Handoff: analysis/handoffs/round-04-commentary-final.md.

### Coordinator — Round 4 final (2026-11-19)

- Fallback worker: regions 1907-1909 (Data, Manual, conf 0.3) at 0x14CCA, 0x17BC6, 0x1FD5A; checkpoint pass-004 ok,
  byte-identical. Commentary worker: final strict editorial audit PASSED (1909/1909, 2117/2117, editorial_audit true);
  coordinator re-ran commentary.py verify (byte-identical, game.asm 712a9ad5...) and archive.editorial_status (complete).
- ROM classification: 100.00% (393232 / 393232). Evidence-backed 393229 B; Manual user-directed fallback 3 B
  (unknown meaning, NOT evidence-backed; explicit user authorization). L018 blocked / L006 assisted remain as recorded.
- Final conclusions: analysis/handoffs/final-round-04.md (final-terminal.md retained as historical record).
- archive-options.json updated: adds final-round-04.md, round-04-fallback.md, round-04-commentary-final.md;
  omits analysis/round-3/commentary/*.md (superseded snapshots / toolkit boilerplate; files retained). No archive run.
