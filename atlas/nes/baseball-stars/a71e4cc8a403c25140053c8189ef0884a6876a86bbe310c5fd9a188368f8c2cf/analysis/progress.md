# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Coordinator log

- 2026-11-14 — Round 1, phase: disassembly (not started). Coordinator read goal.md,
  workflow.md, prompts/disassembly.txt and workspace.json (mapper 1, PRG 128 KiB,
  CHR 128 KiB). BLOCKED before any phase work: every Bash invocation (including
  `true`) in both the coordinator and a launched disassembly worker fails with
  `apply-seccomp: write /proc/self/setgroups (nested userns is capability-restricted;
  caller must provide CAP_SYS_ADMIN): Permission denied`. `python3 run.py inspect/
  baseline/serve` cannot be run, so no baseline, state, map or checkpoint exists.
  No workspace files other than this log were changed. This is a host sandbox
  limitation, not a romatlas toolkit issue.
- Resume point: Round 1, phase disassembly, from scratch (`python3 run.py inspect`
  then `python3 run.py baseline`). No checkpoints, leads consumed/produced or
  handoffs yet. ROM classification not yet measured (no baseline).
- 2026-11-16 — Coordinator resumed. Bash/sandbox now works (prior seccomp block
  cleared). Round 1, phase disassembly: launching worker (from scratch; baseline absent).
- 2026-11-16 20:42 — Coordinator resumed again. Prior round-1 disassembly worker (16:26–16:46)
  produced baseline (generated/baseline, maps/baseline.json, analysis/baseline-verification.json
  byte_identical=true) and scratch probes but left NO handoff; analysis/state.json still has 0
  regions. An orphaned broker (scratch/broker.py, socket /tmp/romatlas-a71e.sock) + romatlas serve
  are still alive and may hold unsaved in-memory state. Relaunching Round 1 disassembly worker
  (resume, not re-init): first recover/save any in-memory state from the orphan server, then shut it down.

## Round 1 disassembly (worker 2, 2026-11-16 ~20:45)

- Orphan broker/serve (pids 1318465/1318467/1318468) queried via socket: map.get total=0
  (no unsaved in-memory state). Shut down cleanly with `__quit__`. New broker started
  (scratch/broker.py, socket /tmp/romatlas-a71e.sock); map.load analysis/state.json (0 regions).
- Game identified from title screen: Baseball Stars (SNK, 1989). MMC1, reset writes control $1E
  (16 KiB PRG mode, $C000-$FFFF fixed = 8 KiB banks 14/15; CHR 4 KiB mode). Vectors (bank 15):
  NMI $CFB3, RESET $CC69, IRQ $D026.
- Fixed-bank inline-argument routines (stack-return): $C53A (2-byte ptr -> $04/$05),
  $C560 (2-byte ptr -> $0C/$0D), $C0EE (variable stream, $FE/$FF terminators), $C178
  (jump table after JSR, A = index; never returns to fallthrough). Far-call trampoline $C18F
  (A = MMC1 PRG value, Y = index into word table at $FF80).
- 21:30 interim checkpoint pass-001 (maps/pass-001.json, analysis/pass-001-verification.json,
  byte_identical=true): 626 regions; Code 42,681 / Data 64,639 / Image (CHR) 131,072; 23,768 unknown.
  Correction found during the pass: MMC1 16K mode PRG value n selects 16 KiB bank n -> 8K banks 2n/2n+1
  (verified against observed contexts: 43 agreeing static bank-tracked call targets, 0 conflicts).
  Map server is now the second, store-less serve instance (socket /tmp/romatlas-a71e-2.sock); the
  first instance (run.py serve) is used only for emulation and never saves.
- 22:00 checkpoint pass-002 (maps/pass-002.json, analysis/pass-002-verification.json, byte_identical=true):
  871 regions; ROM classification 95.25% (249716 / 262160; 12444 unknown). CPU-vs-CPU game reached
  game end (box score AD 12 - BW 1, HOMERUN list) at ~91k frames. Data claims from pass-001/interim
  map: 0 bytes contradicted by the later coverage (18,858 unique observed instruction starts).
- 22:05 FINAL checkpoint pass-003 (maps/pass-003.json, analysis/pass-003-verification.json,
  generated/pass-003, byte_identical=true; map sha256 0408226e...). 871 regions.

ROM classification: 96.38% (252677 / 262160 bytes classified; 9483 bytes unknown).

| Area | Code | Data | Image | Classified | Unknown |
|---|---|---|---|---|---|
| header | 0 | 16 | 0 | 16 | 0 |
| PRG00 | 1635 | 6533 | 0 | 8168 | 24 |
| PRG01 | 2816 | 5274 | 0 | 8090 | 102 |
| PRG02 | 3090 | 3211 | 0 | 6301 | 1891 |
| PRG03 | 3911 | 2426 | 0 | 6337 | 1855 |
| PRG04 | 0 | 8192 | 0 | 8192 | 0 |
| PRG05 | 1187 | 7003 | 0 | 8190 | 2 |
| PRG06 | 0 | 8192 | 0 | 8192 | 0 |
| PRG07 | 2922 | 2691 | 0 | 5613 | 2579 |
| PRG08 | 0 | 8192 | 0 | 8192 | 0 |
| PRG09 | 5454 | 2671 | 0 | 8125 | 67 |
| PRG10 | 1610 | 6498 | 0 | 8108 | 84 |
| PRG11 | 6541 | 1029 | 0 | 7570 | 622 |
| PRG12 | 6898 | 1029 | 0 | 7927 | 265 |
| PRG13 | 6874 | 1232 | 0 | 8106 | 86 |
| PRG14 | 7269 | 779 | 0 | 8048 | 144 |
| PRG15 | 4392 | 2038 | 0 | 6430 | 1762 |
| CHR (16 x 8 KiB) | 0 | 0 | 131072 | 131072 | 0 |
| Total | 54599 | 67006 | 131072 | 252677 | 9483 |

  Provenance: Code Observed-only regions 9,638 B, Code Static (mixed; contains observed starts) 44,961 B;
  Data Static 67,006 B (generic 65.5k, inline args ~0.8k, $C178 jump tables ~0.4k, header/vectors/FF80
  tables); Image Static 131,072 B (CHR). Execution coverage (separate metric): 19,663 unique observed
  instruction starts (~42k bytes) across scenarios S01-S11. Status: INCOMPLETE (unknown bytes remain;
  no commentary/editorial review yet). Toolkit findings: TK-001, TK-002 (missing features, non-blocking),
  see analysis/toolkit-issues.md. Servers started in this pass were stopped. Handoff:
  analysis/handoffs/round1-disassembly.md.

## Coordinator log (continued)

- 2026-11-16 — Round 1 disassembly COMPLETE (pass incomplete by design). Checkpoint pass-003
  (byte-identical). ROM classification: 96.38% (252677 / 262160 bytes classified; 9483 bytes unknown).
  Leads produced L-R1-01..07. Handoff analysis/handoffs/round1-disassembly.md. Printed % to user.
- Round 1, phase commentary: launching worker (commentary/ not yet prepared; use
  /root/romatlas/scripts/commentary.py prepare on pass-003).
- 2026-11-17 — Coordinator resumed. Round 1 commentary worker (22:03-22:18) was interrupted without
  handoff: commentary/ prepared from pass-003, partial annotations (00-hardware, bank14a-d, ram),
  tools in commentary/tools, last verification byte_identical=true. Relaunching Round 1 commentary
  worker in RESUME mode (preserve existing annotations/tools; do not re-prepare).
- 2026-11-17 — Round 1 commentary worker 2 finished: all banks named/commented at routine/data/RAM
  level, review.json 990/990 reviewed, verify byte_identical=true, reading copy refreshed. Handoff
  analysis/handoffs/round1-commentary.md. Coordinator gate: NOT accepted as complete — residual naming
  gaps (394 address-only branch locals; raw-address rarely-used RAM; data banks 4/6/8 table-level only).
  Resuming commentary phase (worker 3) on that exact remaining backlog before redirect.
- 2026-11-17 23:51 — Coordinator resumed. Commentary worker 3 (02:16-02:22) was interrupted without
  handoff: it added annotations/zy-locals.ann (address-only branch locals now 0; verify byte_identical=true
  at 02:22) and started tools/rawram.py (raw RAM backlog). Relaunching commentary worker 4 in RESUME mode
  for remaining backlog: raw-address rarely-used RAM, data banks 4/6/8 table-level descriptions,
  readability assessment, fresh verify + reading copy, updated handoff.
- 2026-11-18 — Commentary worker 4 finished: address-only locals 0, raw RAM operands 0 (ram3.ann +126 symbols),
  screen-map/metatile/field-map/animation/team/sound data formats documented (zw-data.ann), cross-bank
  operand comments (zx-xbank.ann). verify byte_identical=true, reading copy refreshed. Coordinator gate:
  ACCEPTED (spot-checked counts). Round 1 commentary COMPLETE. New redirect leads: Unknown ranges inside
  320-byte map/metatile runs (banks 1/2/10); uncalled maps likely TRADE/HIRE/REMOVE/FIRE states; $2C debug hold.
- Round 1, phase redirect: launching worker (attempt id r1-redirect-01, analysis/redirect/r1-redirect-01/).
- 2026-11-18 02:48 — Coordinator resumed. Redirect worker r1-redirect-01 (00:19-00:50) was interrupted
  without handoff. Saved: notes.md, proposal-static.json (L-R2-01/02/03/05 static proposals, 0 misaligned),
  residual-ledger.json, residual-after-proposals.txt, field-side substitution coverage (cov-fieldside.json),
  MAKE TEAM natural route + menu1team.snap (HIRE/REMOVE/FIRE enabled). Orphan broker (pid 1379953) + serve
  (pid 1379956, started 00:26) still alive. Relaunching redirect worker in RESUME mode for the same attempt
  id r1-redirect-01 (continue from notes.md frontier; do not replay completed experiments).
- 2026-11-18 03:50 — Round 1 redirect (r1-redirect-01 resume) COMPLETE. Orphan servers checked (no map edits) and
  stopped. Runtime: REMOVE/FIRE/HIRE/TRADE menus, VS pre-game options, natural league CHAMPION (MS 40/41) via
  2-team CPU league; 0 misaligned starts vs static proposals. Ready leads L-R2-01..07, L-R2-09 (~9215 of 9483
  Unknown bytes); L-R2-08 (268 bytes, no caller) deferred. scenarios S12/S13 visited, S14-S16 added.
  Handoff analysis/handoffs/round1-redirect.md. No map changes.
- 2026-11-18 03:52 — Coordinator: redirect handoff accepted (ready leads with evidence+recipes). L-R2-08 deferral
  reviewed: reason is concrete negative evidence (no reference/selector, no fetch), not scope; accepted for now.
  Round 2, phase disassembly: launching worker (input checkpoint maps/pass-003.json; leads L-R2-01..07, L-R2-09).

## Round 2 disassembly — pass-004 (2026-11-18)

Input checkpoint maps/pass-003.json (sha256 0408226e...). Output: maps/pass-004.json (sha256
c1ceb883b29f4cc8425b459a8c16b0f92bc8d354ba75f481afef252e6ee148db), generated/pass-004,
analysis/pass-004-verification.json: ok=true, byte_identical=true. analysis/state.json == maps/current.json ==
pass-004 content; map.validate ok.

ROM classification: 99.91% (261915 / 262160 bytes classified; 245 bytes unknown).

| Area | Code | Data | Image | Classified | Unknown |
|---|---|---|---|---|---|
| header | 0 | 16 | 0 | 16 | 0 |
| PRG00 | 1635 | 6557 | 0 | 8192 | 0 |
| PRG01 | 2816 | 5376 | 0 | 8192 | 0 |
| PRG02 | 4762 | 3430 | 0 | 8192 | 0 |
| PRG03 | 5498 | 2593 | 0 | 8091 | 101 |
| PRG04 | 0 | 8192 | 0 | 8192 | 0 |
| PRG05 | 1187 | 7005 | 0 | 8192 | 0 |
| PRG06 | 0 | 8192 | 0 | 8192 | 0 |
| PRG07 | 5111 | 3054 | 0 | 8165 | 27 |
| PRG08 | 0 | 8192 | 0 | 8192 | 0 |
| PRG09 | 5454 | 2719 | 0 | 8173 | 19 |
| PRG10 | 1610 | 6581 | 0 | 8191 | 1 |
| PRG11 | 6989 | 1203 | 0 | 8192 | 0 |
| PRG12 | 6927 | 1239 | 0 | 8166 | 26 |
| PRG13 | 6874 | 1317 | 0 | 8191 | 1 |
| PRG14 | 7275 | 847 | 0 | 8122 | 70 |
| PRG15 | 5913 | 2279 | 0 | 8192 | 0 |
| CHR (16 x 8 KiB) | 0 | 0 | 131072 | 131072 | 0 |
| Total | 62051 | 68792 | 131072 | 261915 | 245 |

Provenance: Code Observed 10,804 B, Code Static 51,247 B, Data Static 68,792 B, Image Static 131,072 B.
Leads: verified/consumed L-R2-01, -02, -03, -05, -06, -07, -09; L-R2-04 blocked (TK-003); L-R2-08 updated
(b3 $B39E-$B3AE corrected to Code by far-call return contract, then observed; 240 bytes remain deferred);
produced L-R3-01 (4 ambiguous bytes); none rejected. Gameplay: new S17 2PLAYER route (natural inputs).
Execution coverage is separate (S17: 9687 starts, 0 in Unknown/Data, 0 misaligned).
Status: INCOMPLETE (245 Unknown bytes). Toolkit: TK-001, TK-002, new TK-003 (missing features, non-blocking
except TK-003 blocks L-R2-04's byte), see analysis/toolkit-issues.md. Servers stopped.
Handoff analysis/handoffs/round2-disassembly.md.
- 2026-11-18 04:25 — Coordinator: Round 2 disassembly accepted. Checkpoint pass-004 (sha256 c1ceb883..., byte_identical).
  ROM classification: 99.91% (261915 / 262160 bytes classified; 245 bytes unknown) — printed to user.
  Leads consumed L-R2-01/02/03/05/06/07/09; produced L-R3-01; L-R2-04 blocked (TK-003); L-R2-08 240 B deferred.
  Round 2, phase commentary: launching worker. Map changed vs commentary baseline (pass-003) -> preserve previous
  commentary under analysis/round-1/ before re-prepare, carry names/comments by physical range.
- 2026-11-18 — Round 2 commentary: COMPLETE (current-source readability). Map changed (pass-003 -> pass-004):
  previous commentary preserved in analysis/round-1/commentary/, fresh baseline prepared from pass-004
  (c1ceb883...), annotations carried by physical range (2995 names landed; affected records rewritten).
  New code commented: fielding-side substitution pages (b2), pre-game option pages / END PLAY / result prize
  roll (b3), REMOVE/FIRE/HIRE (b7), league champion (b11), runner state 5 (b12), bank 14 prologues, TRADE (b15).
  Counts: 0 auto labels, 0 address-only locals, 0 raw RAM operands, 5 fall-through labels without own head;
  review.json 1224/1224 reviewed, 1395 units; editorial_audit pending. verify byte_identical=true; reading copy
  refreshed. Corrections: CustomTeamFlags polarity, LeagueSlotStats $FF = fired slot, FieldChangeCount,
  MapTablePtrs callers / bank 3 $BC30 champion maps. Leads produced: L-C2-01, L-C2-02 (low).
  Handoff analysis/handoffs/round2-commentary.md.
- 2026-11-18 04:45 — Coordinator: Round 2 commentary gate ACCEPTED (spot-check: 0 auto labels in source,
  verification.json byte_identical=true, reading-after.asm refreshed 04:41, round-1 commentary preserved in
  analysis/round-1/commentary/). Leads produced L-C2-01, L-C2-02.
  Round 2, phase redirect: launching worker (attempt r2-redirect-01) on residual 245 B: L-R2-08 (240), L-R3-01 (4),
  L-R2-04 (1, TK-003), plus L-C2-01/02. Must include compact rejection review if terminal.
- 2026-11-18 — Round 2 redirect (r2-redirect-01): READY-FOR-DISASSEMBLY. Input pass-004 (c1ceb883..., 245 Unknown).
  New ready lead L-R4-01: 240 B (238 Code + 2 inline Data) for the seven L-R2-08 fragments on corrected premise
  (static routine-contract / byte-identical-duplicate evidence; reachability unresolved, entry re-search negative).
  L-R2-08 superseded; L-R3-01 (4 B) deferred with rejection review; L-R2-04 (1 B) blocked TK-003; L-C2-01 closed
  (static MenuStep writer enumeration), L-C2-02 closed (S08). Scenario review S18 (no unvisited milestone).
  Map/ROM/source/toolkit unchanged; no servers. Handoff analysis/handoffs/round2-redirect.md.
- 2026-11-18 05:00 — Coordinator: Round 2 redirect accepted (ready lead L-R4-01, 240 B, per-fragment contract evidence;
  L-C2-01/02 closed; L-R3-01 4 B deferred with reasons; L-R2-04 blocked TK-003). Round 3, phase disassembly: launching
  worker (input pass-004; lead L-R4-01).

## Round 3 disassembly — pass-005 (2026-11-18)

Input checkpoint maps/pass-004.json (sha256 c1ceb883...) == analysis/state.json. Output: maps/pass-005.json
(sha256 407f32c4ca37db1bc813060b933f91c3ea6001be92a2eddd4a4959b8eea92ee1), generated/pass-005,
analysis/pass-005-verification.json: ok=true, byte_identical=true. analysis/state.json == maps/current.json ==
pass-005; map.validate ok. input/game.nes unchanged (a71e4cc8...).

ROM classification: 100.00% (262155 / 262160 bytes classified; 5 bytes unknown).
(Exact 99.998%; the rounded 100.00% does NOT indicate completion — 5 Unknown bytes remain.)

| Area | Code | Data | Image | Classified | Unknown |
|---|---|---|---|---|---|
| header | 0 | 16 | 0 | 16 | 0 |
| PRG00 | 1635 | 6557 | 0 | 8192 | 0 |
| PRG01 | 2816 | 5376 | 0 | 8192 | 0 |
| PRG02 | 4762 | 3430 | 0 | 8192 | 0 |
| PRG03 | 5597 | 2595 | 0 | 8192 | 0 |
| PRG04 | 0 | 8192 | 0 | 8192 | 0 |
| PRG05 | 1187 | 7005 | 0 | 8192 | 0 |
| PRG06 | 0 | 8192 | 0 | 8192 | 0 |
| PRG07 | 5138 | 3054 | 0 | 8192 | 0 |
| PRG08 | 0 | 8192 | 0 | 8192 | 0 |
| PRG09 | 5473 | 2719 | 0 | 8192 | 0 |
| PRG10 | 1610 | 6581 | 0 | 8191 | 1 |
| PRG11 | 6989 | 1203 | 0 | 8192 | 0 |
| PRG12 | 6950 | 1239 | 0 | 8189 | 3 |
| PRG13 | 6874 | 1317 | 0 | 8191 | 1 |
| PRG14 | 7345 | 847 | 0 | 8192 | 0 |
| PRG15 | 5913 | 2279 | 0 | 8192 | 0 |
| CHR (16 x 8 KiB) | 0 | 0 | 131072 | 131072 | 0 |
| Total | 62289 | 68794 | 131072 | 262155 | 5 |

Provenance: Code Observed 10,804 B, Code Static 51,485 B, Data Static 68,794 B, Image Static 131,072 B.
Change vs pass-004: +240 B (238 Code, 2 inline Data) from lead L-R4-01, all seven fragments accepted
independently (b3 $AC36-$AC46, $AC73-$AC97, $B496-$B4C4 incl. inline .addr $B4A9-$B4AA; b7 $B820-$B83A;
b9 $ACC2-$ACD4; b12 $8DE7-$8DFD; b14 $C126-$C16B). Static provenance; reachability unresolved / no known
caller (not Observed, not claimed dead). Remaining Unknown: b10 $9B08 (1, L-R3-01), b12 $9026-$9028 (3, L-R3-01),
b13 $A07A (1, L-R2-04 blocked by TK-003). No gameplay run this pass (S01-S18 exhausted for gains).
Status: INCOMPLETE (5 Unknown bytes). Toolkit: TK-001..TK-003 unchanged, see analysis/toolkit-issues.md.
Servers started this pass were stopped. Handoff analysis/handoffs/round3-disassembly.md.
- 2026-11-18 05:10 — Coordinator: Round 3 disassembly accepted. Checkpoint pass-005 (sha256 407f32c4..., byte_identical).
  ROM classification: 100.00% (262155 / 262160 bytes classified; 5 bytes unknown) [exact 99.998%] — printed to user.
  L-R4-01 consumed (7/7 fragments accepted, Static, reachability unresolved). Residual: L-R3-01 (4 B), L-R2-04 (1 B, TK-003).
  Round 3, phase commentary: launching worker (map changed vs pass-004 baseline -> preserve round-2 commentary under
  analysis/round-2/ then carry forward).
- 2026-11-18 05:10 — Round 3 commentary COMPLETE on pass-005 (407f32c4...). Round-2 commentary preserved in
  analysis/round-2/commentary/; fresh baseline (1226 regions), all names carried, only the 7 L-R4-01 notes + 3
  Unknown notes invalidated/rewritten (annotations/r3-commentary.ann). New: SetupSprite0Split3_Copy,
  TileXYToPpuAddr_B3, MainMenu_EraseCursorItem, MarkTeamEmptyIfAllFired, PutHexBlankPadded, RunnerSafeAtThrowBase,
  PpuBufAppendHexInline (no known caller). Fixes: TileXYToPpuAddr Y*64, LogoUsed->LogoFree (inverted meaning),
  bank 3 grid/team-select scratch names, OPC_RTS overlap operand. Counts: 0 auto labels, 0 address-only locals,
  0 raw RAM, review 1226/1226 + 1404 units (audit pending). verify byte_identical=true; reading copy refreshed.
  Handoff analysis/handoffs/round3-commentary.md.
- 2026-11-18 05:25 — Coordinator: Round 3 commentary gate ACCEPTED (0 auto labels/address-only locals/raw RAM; verify
  byte_identical; reading copy refreshed; round-2 commentary preserved in analysis/round-2/commentary/). No new structural
  leads. Round 3, phase redirect: launching worker (attempt r3-redirect-01) on residual 5 B (L-R3-01, L-R2-04) plus
  low-value $B820 FIRE-last-player trace; terminal requires compact rejection review.
- 2026-11-18 — Redirect r3-redirect-01 complete: READY-FOR-DISASSEMBLY. L-R2-04 reopened (corrected premise:
  extend Code region 3514 back to $A07A; scratch validate/generate/verify byte_identical) and new L-R5-01
  (b12 $9026-$9028 Code via ObjState/body pairing) -> 4 B ready (proposal-r3.json). b10 $9B08 kept Unknown
  (rejection review in analysis/handoffs/round3-redirect.md). TK-003 note appended. $B820 FIRE trace skipped
  (no reference; reachability-only). No server; scratch removed.
- 2026-11-18 05:40 — Coordinator: Round 3 redirect accepted. Ready: L-R2-04 (1 B, corrected premise: extend code region
  back to $A07A; $A07B observed entry recorded as evidence/`*-1` label) and L-R5-01 (3 B b12 $9026, Static, unreachable as
  written). b10 $9B08 (1 B) kept Unknown with rejection review in analysis/handoffs/round3-redirect.md.
  Round 4, phase disassembly: launching worker (input pass-005).

## Round 4 disassembly — pass-006 (2026-11-18)
Input maps/pass-005.json (407f32c4...). Output maps/pass-006.json (sha256 ac37c43fff6441132a9919788f6769764a5c76829235748e4edc86cbc982f073),
generated/pass-006, analysis/pass-006-verification.json: ok=true, byte_identical=true. state.json == current.json == pass-006.

ROM classification: 100.00% (262159 / 262160 bytes classified; 1 bytes unknown).
(Exact 99.9996%; rounded 100.00% does NOT indicate completion — 1 Unknown byte remains: b10 $9B08, phys $15B18.)

| Area | Code | Data | Image | Classified | Unknown |
|---|---|---|---|---|---|
| header | 0 | 16 | 0 | 16 | 0 |
| PRG00 | 1635 | 6557 | 0 | 8192 | 0 |
| PRG01 | 2816 | 5376 | 0 | 8192 | 0 |
| PRG02 | 4762 | 3430 | 0 | 8192 | 0 |
| PRG03 | 5597 | 2595 | 0 | 8192 | 0 |
| PRG04 | 0 | 8192 | 0 | 8192 | 0 |
| PRG05 | 1187 | 7005 | 0 | 8192 | 0 |
| PRG06 | 0 | 8192 | 0 | 8192 | 0 |
| PRG07 | 5138 | 3054 | 0 | 8192 | 0 |
| PRG08 | 0 | 8192 | 0 | 8192 | 0 |
| PRG09 | 5473 | 2719 | 0 | 8192 | 0 |
| PRG10 | 1610 | 6581 | 0 | 8191 | 1 |
| PRG11 | 6989 | 1203 | 0 | 8192 | 0 |
| PRG12 | 6953 | 1239 | 0 | 8192 | 0 |
| PRG13 | 6875 | 1317 | 0 | 8192 | 0 |
| PRG14 | 7345 | 847 | 0 | 8192 | 0 |
| PRG15 | 5913 | 2279 | 0 | 8192 | 0 |
| CHR (16 x 8 KiB) | 0 | 0 | 131072 | 131072 | 0 |
| Total | 62293 | 68794 | 131072 | 262159 | 1 |

Provenance: Code Observed 10,804 B, Code Static 51,489 B, Data Static 68,794 B, Image Static 131,072 B.
Change vs pass-005: +4 B Code. L-R2-04 verified (region 3514 extended to $A07A; $A07B alternate entry documented in
evidence, TK-003 workaround). L-R5-01 verified (b12 $9026-$9028 `jmp RunnerMoveAlongPath`, Static, not reachable as
written). L-R3-01 ($9B08) deferred unchanged. No gameplay run. Status: INCOMPLETE (1 Unknown byte).
Toolkit: TK-001..TK-003 (TK-003 update appended), see analysis/toolkit-issues.md. Server started this pass was stopped.
Handoff analysis/handoffs/round4-disassembly.md.
- 2026-11-18 05:50 — Coordinator: Round 4 disassembly accepted. Checkpoint pass-006 (sha256 ac37c43f..., byte_identical).
  ROM classification: 100.00% (262159 / 262160 bytes classified; 1 bytes unknown) [b10 $9B08] — printed to user.
  L-R2-04 and L-R5-01 consumed. Round 4, phase commentary: launching worker (preserve round-3 commentary under
  analysis/round-3/, carry forward to pass-006).
- 2026-11-18 — Round 4 commentary: COMPLETE on pass-006 (ac37c43f...). Round-3 commentary preserved in
  analysis/round-3/commentary/ (diff -r identical); fresh prepare (1225 regions), annotations carried by
  physical range. New: b13 CpuThrow_ThrowIfBaseNear + `FieldUpdateThrowEntry = * + 1` (overlapping $A07B entry,
  jsr in FieldCommonUpdate now symbolic, TK-003 explained), b12 RunnerInput_ReturnOldTail, b10 $9B08 note updated;
  scoped menu scratch names replace all Ball* operands outside field-play code. 0 auto labels, 0 address-only
  locals, 0 raw RAM operands; review.json 1225/1225 reviewed, 1405 units, editorial_audit pending.
  commentary.py verify ok/byte_identical=true; reading-after.asm refreshed. Handoff analysis/handoffs/round4-commentary.md.
- 2026-11-18 06:20 — Coordinator: Round 4 commentary gate ACCEPTED (0 auto labels/locals/raw RAM; verify byte_identical;
  round-3 commentary preserved in analysis/round-3/). Residual complement refreshed from state.json: exactly phys $15B18
  (b10 $9B08, 1 B). Coordinator inspected the r3-redirect-01 rejection review for $9B08: sound, premises unchanged by
  pass-006 (edits only in banks 12/13); no corrected inference or discriminating test remains -> no Round 4 redirect
  relaunch (reuse of sound review per goal.md). TERMINAL INCOMPLETE (not whole-ROM completion).
  Final fresh commentary verify byte_identical=true; reading copy refreshed. editorial_audit pending by design.
  ROM classification: 100.00% (262159 / 262160 bytes classified; 1 bytes unknown) [exact 99.9996%].
  archive-options.json written (15 documents, 15 omitted superseded round-N commentary copies). No archive run.
  Final handoff: analysis/handoffs/final-terminal.md. Resume point: only with new evidence for $9B08, then final
  editorial audit + `commentary.py audit` + archive.

## Batch exception — user-directed Manual fallback for residual bytes — pass-007 (2026-11-18)
Explicit user authorization (batch-only exception, valid only when at most 10 physical ROM bytes remain unclassified):
mark the listed residual bytes as generic Data with UNKNOWN MEANING using map.annotate with Manual provenance. This is a
user-directed fallback, NOT an observed or statically proven data role, and does not reopen redirect research.
- Complement verified first from analysis/state.json (pass-006, ac37c43f...) via analysis.unknown: exactly one range,
  phys offset 88856 ($15B18), length 1 — matches the user's validated list [{"offset": 88856, "length": 1}] (1 <= 10).
- Location: PRG bank 10, CPU $9B08 (context [10,11,14,15]); between NameGrid_Right's `jmp PlaceGridCursor` (ends $9B07)
  and Data table GridRightWrap ($9B09-$9B0D, region 3920). ROM byte unchanged.
- Prior ambiguity (lead L-R3-01; rejection review analysis/handoffs/round3-redirect.md; coordinator inspection in
  analysis/handoffs/final-terminal.md): no reference, branch, fall-through or fetch; leftover `rts` after a jsr->jmp edit
  and pad byte both remain consistent; a lone unreferenced return opcode gives insufficient structure for Code and a
  single byte gives no repeated-pattern basis for Static Data. That code-vs-data ambiguity remains UNRESOLVED.
- Action: map.annotate new region id 3989, offset 88856, length 1, kind Data, provenance Manual, confidence 0.5, no
  label (no semantics invented); evidence/comment record the fallback and this authorization. map.validate ok,
  map.save analysis/state.json, map.export maps/current.json. Pre-fallback store copy: scratch/r5/state-pre-fallback.json
  (pass-006 map retained as maps/pass-006.json). Scripts: scratch/r5/apply.py.
- Checkpoint: `run.py checkpoint pass-007` -> maps/pass-007.json (sha256 d0dd6e1f91d04845b83dd6c8c795603f08456c4aafef2aa83be775e1d3942bdf)
  == maps/current.json == analysis/state.json; generated/pass-007; analysis/pass-007-verification.json ok=true,
  byte_identical=true. input/game.nes unchanged (a71e4cc8...).

ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).
Of these, 262159 B are evidence-backed (Code Observed 10,804, Code Static 51,489, Data Static 68,794, Image Static
131,072) and 1 B (phys $15B18) is the user-directed Manual fallback (generic Data, meaning unknown) — reported separately;
it is NOT an evidence-backed classification. PRG10: Code 1610, Data 6582 (incl. 1 Manual), Unknown 0.
- Next: Round 5 commentary / final acceptance worker (reconcile commentary inventory with pass-007, finish commentary,
  final review + strict editorial audit, fresh byte-identical verification), then archive-options.json update.

## Round 5 commentary / final acceptance — pass-007 (2026-11-18, complete)
- Round-4 commentary preserved in analysis/round-4/commentary/ (diff -r identical); fresh baseline from pass-007
  (d0dd6e1f...), 1225 regions; annotations/tools/notes/progress/uncertainties carried by physical range. Only the
  b10 $9B08 note was invalidated; it is now `.byte $60` with a note on the user-directed Manual fallback (meaning
  unknown, L-R3-01 open).
- Final editorial review: wrong name fixed (SubMenuTextPtrs/DrawSubMenuText), pointer tables structured in banks 0/2/3/9,
  TeamBufPtrs boundary split, FF80 entries 16-24 named, certainty-loaded RAM names softened. See
  analysis/handoffs/round5-commentary-final.md.
- `commentary.py audit` ok=true, byte_identical=true, 1225 regions / 1411 units reviewed, editorial_audit reviewed;
  `commentary.py verify` byte_identical=true; reading-after.asm refreshed. input/game.nes unchanged.
- 2026-11-18 — Coordinator: Round 5 commentary/final acceptance ACCEPTED (analysis/handoffs/round5-commentary-final.md).
  Independent re-check: maps/current.json == state.json == pass-007 (d0dd6e1f...), input/game.nes unchanged;
  commentary/audit.json ok/byte_identical/editorial_audit=true (1225/1225 regions, 1411/1411 units); fresh
  `commentary.py verify` ok=true byte_identical=true; reading copy refreshed; archive.editorial_status complete=true.
  ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown) — 262,159 B evidence-backed +
  1 B user-directed Manual fallback (phys $15B18, generic Data, meaning unknown; NOT evidence-backed).
  archive-options.json updated (17 documents, 20 omitted superseded round-N commentary copies, fallback_record;
  every analysis/ and commentary/ Markdown accounted for). No archive/cleanup/publish run.
  Final handoff: analysis/handoffs/final-acceptance.md (supersedes final-terminal.md, which is retained).
