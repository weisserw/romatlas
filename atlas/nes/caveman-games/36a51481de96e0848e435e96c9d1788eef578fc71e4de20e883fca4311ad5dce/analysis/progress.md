# Disassembly progress

## Orchestration (coordinator; goal.md)

- Round: 5
- Current phase: TERMINAL HANDOFF — whole-ROM classification and final editorial acceptance complete (2026-11-25); ready for archive (not executed)
- Final checkpoint: pass-005 (maps/pass-005.json; analysis/pass-005-verification.json)
- Phase handoffs: analysis/handoffs/round-NN-<phase>.md
- Disk at start: ~2.9G free on / (90% used) — keep scratch bounded.

### Phase log
| Round | Phase | Status | Input ckpt | Output ckpt | Verification | Leads consumed/produced | Classification |
|---|---|---|---|---|---|---|---|
| 1 | disassembly | done (incomplete) | baseline | pass-001b | byte-identical | produced L-001..L-006 | 99.12% (259842/262160; 2318 unknown) |
| 1 | commentary (part 1) | interrupted: banks 0,1,7 done; banks 2-6 remaining (commentary/pending_units.json) | pass-001b | — | commentary/verification.json byte-identical | produced L-007..L-010 | 99.12% |
| 1 | commentary (part 2) | done: all classified source readable, pending_units empty | pass-001b | — | commentary/verification.json byte-identical; reading copy commentary/reading-after.asm | produced L-011 | 99.12% |
| 1 | redirect | done: recommend (a) new round | pass-001b | — | n/a (no map change) | ready L-001,L-010,L-008,L-009,L-007,L-011,L-012; closed L-006; deferred L-004,L-005 | 99.12% |
| 2 | disassembly | done | pass-001b | pass-002 | byte-identical (analysis/pass-002-verification.json) | accepted L-001,L-007,L-008,L-009,L-010,L-011,L-012; open L-004 (reachability), L-005 deferred | 100.00% (262160/262160; 0 unknown) |
| 2 | commentary | done: all classified source readable (843 units, 5548 regions reviewed); editorial_audit pending | pass-002 | — | commentary/verification.json byte-identical; reading copy refreshed; round-1 commentary preserved at analysis/round-1/commentary/ | none new | 100.00% |
| 2 | redirect | done: recommend (a) correction round | pass-002 | — | n/a; whole-map consistency check clean (analysis/redirect/r02-a1/) | ready L-013..L-016, optional L-017,L-018; S15 multi-player visited | 100.00% |
| 3 | disassembly | done | pass-002 | pass-003 | byte-identical (analysis/pass-003-verification.json) | accepted L-013,L-014,L-016,L-017,L-018; L-015 left for commentary | 100.00% (262160/262160; 0 unknown) |
| 3 | commentary | done: all classified source readable (857 units, 5551 regions); editorial_audit pending; round-2 copy at analysis/round-2/commentary/ | pass-003 | — | commentary/verification.json byte-identical; reading copy refreshed | consumed L-015 | 100.00% |
| 3 | redirect | done: recommend (a) correction round | pass-003 | — | n/a; static code audit clean (analysis/redirect/r03-a1/) | ready L-019, L-020; deferred L-004, L-005 rest, S16 | 100.00% |
| 4 | disassembly | done | pass-003 | pass-004 | byte-identical (analysis/pass-004-verification.json) | accepted L-019 (125 regions), L-020 (disassembly part); commentary part of L-020 open | 100.00% (262160/262160; 0 unknown) |
| 4 | commentary | done: all classified source readable (857 code + 189 data units, 4306 rows); editorial_audit pending; round-3 copy at analysis/round-3/commentary/ | pass-004 | — | commentary/verification.json byte-identical; reading copy refreshed | consumed L-020; produced L-021 (low) | 100.00% |
| 4 | redirect | done: recommend (a) short round then (c) final acceptance; coordinator reviewed rejection review (sound: no code candidates, no consumers; analysis/redirect/r04-a1/) | pass-004 | — | n/a | ready L-021, L-022 (low); deferred L-004, L-005, S16 | 100.00% |
| 5 | disassembly | done | pass-004 | pass-005 | byte-identical (analysis/pass-005-verification.json) | accepted L-021 (+optional bank 2 split), L-022 | 100.00% (262160/262160; 0 unknown) |
| 5 | commentary | done: all classified source readable (857 code + 192 data units, 4280 rows); round-4 copy at analysis/round-4/commentary/ | pass-005 | — | commentary/verification.json byte-identical; reading copy refreshed | no new leads | 100.00% |
| 5 | redirect | satisfied by reuse: round-4 redirect rejection review (analysis/handoffs/round-04-redirect.md, analysis/redirect/r04-a1/) covered all remaining avenues; round 5 only applied its recommended L-021/L-022 with no new evidence or ready leads, so premises unchanged (goal.md: reuse sound review, no automatic cycling) | pass-005 | — | n/a | none ready; L-004 reachability, L-005 roles deferred | 100.00% |
| final | editorial acceptance | done: commentary.py audit ok (commentary/audit.json: 4280 regions, 1896 units reviewed, editorial_audit true) | pass-005 | pass-005 | byte-identical (audit.json, commentary/verification.json, analysis/pass-005-verification.json); reading copy commentary/reading-after.asm refreshed | — | 100.00% (262160/262160; 0 unknown) |

### Terminal handoff (coordinator)
- ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown). Final map checkpoint pass-005
  (maps/pass-005.json == maps/current.json; analysis/pass-005-verification.json byte-identical).
- Commentary: all classified source readable; final editorial audit passed (commentary/audit.json, handoff
  analysis/handoffs/final-acceptance.md); reading copy commentary/reading-after.asm.
- Explained residual uncertainties (semantic/reachability only, not classification gaps): commentary/uncertainties.md;
  L-004 (unreferenced static routines, reachability), L-005 (data roles without consumers), IrqHandler unobserved,
  S16 natural full Start-the-Games run deferred (analysis/scenarios.json). Toolkit issues: analysis/toolkit-issues.md.
- Last redirect rejection review (round 4) inspected by coordinator and reused for round 5 (unchanged premises).
- Archive selection: archive-options.json (27 final documents: phase handoffs, redirect notes r01-r04, historical
  per-round commentary progress/uncertainties; 12 omitted: per-round copies of commentary AGENTS/prompt/workflow
  instruction boilerplate). Checked against toolkit scripts/archive.py: no unrecognized Markdown remains.
  Archive execution, publication and cleanup NOT performed (not authorized).

## Pass records

### Round 1 disassembly pass (2026-11-24/25) — status: incomplete (easy gains exhausted)

- Baseline: `python3 run.py baseline` -> maps/baseline.json, generated/baseline, analysis/baseline-verification.json (byte-identical).
- Output checkpoint: **pass-001b** (maps/pass-001b.json, generated/pass-001b, analysis/pass-001b-verification.json:
  ok, byte_identical=true, map_sha256 12d216b3...). pass-001 (same pass, before splitting $00 runs into Padding) is superseded but kept.
- Internal store analysis/state.json; export maps/current.json; map.validate ok.

ROM classification: 99.12% (259842 / 262160 bytes classified; 2318 bytes unknown).

| Range | Bytes | Code | Data | Text | Padding | Image | Unknown |
|---|---|---|---|---|---|---|---|
| Header | 16 | 0 | 16 | 0 | 0 | 0 | 0 |
| PRG16 0 (8K 0/1) menus | 16384 | 3820 | 5913 | 4664 | 1987 | 0 | 0 |
| PRG16 1 (8K 2/3) Mate Toss | 16384 | 4707 | 11397 | 52 | 228 | 0 | 0 |
| PRG16 2 (8K 4/5) Fire Starting | 16384 | 4916 | 7439 | 10 | 3973 | 0 | 46 |
| PRG16 3 (8K 6/7) Dino Race | 16384 | 4820 | 8633 | 128 | 543 | 0 | 2260 |
| PRG16 4 (8K 8/9) Saber Race | 16384 | 5614 | 6682 | 0 | 4088 | 0 | 0 |
| PRG16 5 (8K 10/11) Clubbing | 16384 | 7164 | 8989 | 70 | 149 | 0 | 12 |
| PRG16 6 (8K 12/13) Dino Vault | 16384 | 5879 | 8444 | 151 | 1910 | 0 | 0 |
| PRG16 7 (8K 14/15) fixed | 16384 | 4508 | 10415 | 265 | 1196 | 0 | 0 |
| CHR | 131072 | 0 | 0 | 0 | 0 | 131072 | 0 |

- Evidence: code = static recursive descent with MMC1 mode-3 context from vectors, bank stubs, observed roots and 31
  bounded jump tables (Observed-provenance code regions: 30981 bytes; observed instruction starts 13651);
  717 bytes inferred routines (Static, conf 0.8); duplicate reset/MMC1 stubs per bank by byte identity.
  Data/Text/Padding by gap segmentation (see analysis/investigation.md). No observed fetch conflicts with static boundaries.
- Execution coverage (separate metric): 13651 unique observed ROM instruction starts across scenarios S01-S13.
- Scenarios: boot/attract, all menus, all six Practice events, Start the Games (3 events natural, final placings via
  artificial $0659=6 shortcut), intermediate results, Caves of Fame, Restart (analysis/scenarios.json).
- Leads: none consumed (fresh); produced L-001..L-006 (analysis/leads.json).
- Toolkit: 4 findings (TK-001 suspected bug input params; TK-002/TK-003 missing features; TK-004 doc gap), none blocking.
  See analysis/toolkit-issues.md.
- Handoff: analysis/handoffs/round-01-disassembly.md

### Round 1 commentary pass (2026-11-25) — status: INCOMPLETE (event-bank depth remaining)

- Input pass-001b (unchanged map/state; classification still 99.12%, 2318 unknown bytes).
- Prepared commentary/ (commentary.py prepare, frozen before/). Source is rebuilt from notes by
  commentary/tools/build_source.py into game.asm + hw.inc + ram.inc + bank0-7.asm.
- Verification: commentary/verification.json — ok, byte_identical true. Reading copy: commentary/reading-after.asm.
- Coverage: banks 0, 1 and 7 plus all sound-driver/stub copies are fully commented. Banks 2-6 have every routine
  named and described; 192 units are still pending with only one-line summaries (commentary/pending_units.json).
  Ranges (first..last pending unit start, file offsets): bank 2 0x810D-0x9663, bank 3 0xC215-0xD42F, bank 4 0x1010D-0x11787,
  bank 5 0x1411F-0x16928, bank 6 0x18068-0x1B793.
- Leads produced: L-007..L-010 (analysis/leads.json); none consumed.
- Toolkit: TK-005 (missing feature, not blocking).
- Handoff: analysis/handoffs/round-01-commentary.md

### Round 1 commentary continuation (2026-11-25) — status: COMPLETE (current-source readability)

- Same input (pass-001b, unchanged); classification unchanged.
- Covered the 192 pending event-bank units (banks 2-6) plus a whole-source sweep: no auto branch labels,
  numeric work-RAM operands or address-named data labels remain; pending_units.json empty; review.json all reviewed
  (editorial_audit pending for final acceptance).
- Verification: commentary/verification.json ok, byte_identical true; reading copy refreshed.
- Leads produced: L-011 (dead-code-like Data bytes in banks 3/5/6); none consumed.
- Handoff: analysis/handoffs/round-01-commentary.md (Continuation section).

### Round 1 redirect (2026-11-25, attempt analysis/redirect/r01-a1) — status: done, proposals ready

- Input pass-001b; map/state unchanged (99.12%, 2318 unknown). No toolkit change, no publish/archive.
- Proposals for all 2318 unknown bytes: L-001 Data (2254), L-008 Data 2 + Code 44, L-009 Code 6, L-010 Data 12.
  Fidelity corrections: L-007 (bank-7 IRQ split handlers and stubs Data->Code), L-011 (optional), L-012 (provenance).
- Gameplay: Dino Race and Saber Race finished naturally (S14 visited); 537 new instruction starts, all in existing Code.
- Leads: updated L-001..L-011, added L-012 (analysis/leads.json). Ledger: analysis/redirect/r01-a1/unknown_ledger.json.
- Toolkit: no new issues found (existing TK-001..TK-005 unchanged; see analysis/toolkit-issues.md).
- Handoff: analysis/handoffs/round-01-redirect.md


### Round 2 disassembly pass (2026-11-25) — status: all bytes classified; overall completion still pending commentary/editorial gates

- Input pass-001b (analysis/state.json restored via map.load of the edited store; prior store == maps/pass-001b.json).
  No baseline re-init. Edits made by tools/r02_apply.py (pure JSON edit, guarded: only Data regions are replaced;
  every new Code region must decode to its exact end), then map.load -> map.validate ok -> map.save analysis/state.json
  -> map.export maps/current.json.
- Output checkpoint: **pass-002** (maps/pass-002.json, generated/pass-002, analysis/pass-002-verification.json:
  ok, byte_identical=true, map_sha256 b228f49e...). input/game.nes unchanged (sha256 36a51481...).

ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).

| Range | Bytes | Code | Data | Text | Padding | Image | Unknown |
|---|---|---|---|---|---|---|---|
| Header | 16 | 0 | 16 | 0 | 0 | 0 | 0 |
| PRG16 0 (8K 0/1) menus | 16384 | 3820 | 5913 | 4664 | 1987 | 0 | 0 |
| PRG16 1 (8K 2/3) Mate Toss | 16384 | 4707 | 11397 | 52 | 228 | 0 | 0 |
| PRG16 2 (8K 4/5) Fire Starting | 16384 | 4960 | 7441 | 10 | 3973 | 0 | 0 |
| PRG16 3 (8K 6/7) Dino Race | 16384 | 4826 | 10887 | 128 | 543 | 0 | 0 |
| PRG16 4 (8K 8/9) Saber Race | 16384 | 5614 | 6682 | 0 | 4088 | 0 | 0 |
| PRG16 5 (8K 10/11) Clubbing | 16384 | 7175 | 8990 | 70 | 149 | 0 | 0 |
| PRG16 6 (8K 12/13) Dino Vault | 16384 | 5889 | 8434 | 151 | 1910 | 0 | 0 |
| PRG16 7 (8K 14/15) fixed | 16384 | 4619 | 10304 | 265 | 1196 | 0 | 0 |
| CHR | 131072 | 0 | 0 | 0 | 0 | 131072 | 0 |
| Total | 262160 | 41610 | 70064 | 5340 | 14074 | 131072 | 0 |

- Code provenance: Observed 32245 B (was 30981), Static 9365 B (incl. 717 B L-004 inferred roots, still unexecuted,
  and the new unreferenced static routines below). Execution coverage (separate metric): 13651 + 537 = 14188 unique
  observed instruction starts (round 1 + r01-a1 sessions); 0 observed fetches outside Code.
- Lead outcomes (all independently re-checked; details in analysis/leads.json and analysis/investigation.md):
  - L-001 accepted: bank 3 file 0xE61C-0xEF08 one generic Data region DR_StaleForeignImage (2254 unknown + 31 B merged Data), conf 0.85.
  - L-010 accepted: bank 5 0x15580-0x15591 pair table (18 B) + 0x15592-0x15597 placements (6 B), Data conf 0.9 (+12 B).
  - L-008 accepted: bank 2 Data 0x95F6-0x95F7 + Static Code 0x95F8-0x9623 (stack-argument routine, unreferenced) (+46 B).
  - L-009 accepted: bank 3 Static Code 0xD189-0xD18E (+6 B).
  - L-007 accepted: fixed bank 111 B Data->Code (IRQ split tail $C036, handlers $C080-$C0BD, $F26A, $C62F, $C61A, $C5F1);
    IRQ dispatch words $C072-$C07F stay Data (14 B); $CEDB-$CEE7 kept Data.
  - L-011 accepted: 21 B Data->Code (bank 5 $8CBD, bank 6 $AE0C), Static conf 0.7.
  - L-012 accepted: 50 Static Code regions upgraded to Observed, 17 split at observed/static instruction boundaries (1264 B).
  - Rejected: none. L-004 open (reachability only); L-005 deferred; S15 (2-player) deferred.
- Gameplay: no new runs this pass (classification complete; remaining gameplay value is L-004 reachability only;
  last natural runs added 0 bytes outside existing Code). No emulator session or background process left running.
- Verification: map.validate ok; checkpoint pass-002 byte-identical.
- Toolkit: no new issues found (TK-001..TK-005 unchanged; analysis/toolkit-issues.md).
- Not complete overall: commentary for the changed ranges (banks 2/3/5/6/7) and the final editorial audit remain.
- Handoff: analysis/handoffs/round-02-disassembly.md

### Round 2 commentary (2026-11-25) — status: COMPLETE (current-source readability)
- Input pass-002 (100.00% classified; classification unchanged by commentary). Map differed from the round-1
  commentary baseline, so the whole round-1 commentary/ was preserved at analysis/round-1/commentary/ and
  commentary/ was re-prepared (fresh before/ + scaffold, 5548 rows).
- Carry-forward by physical range through commentary/notes (bank + CPU address): 815 code units carried
  unchanged, 9 re-read (extent changed by neighbouring new code), 19 new units written for the changed ranges
  (bank 7 IRQ split handlers/stubs, bank 2 FS_QueueMeterRow, bank 3 DR_SetVelocityBoth + DR_StaleForeignImage,
  bank 5 CL_Bracket6Pairs/Places + CL_IsState2to4, bank 6 DV_FailBelow30). Stale "Unknown" notes removed.
- review.json: 843 code units and all scaffold rows reviewed; editorial_audit pending.
- Verification: commentary.py verify ok, byte_identical true (commentary/verification.json); reading copy refreshed.
- New leads: none. Toolkit: no new issues found.
- Handoff: analysis/handoffs/round-02-commentary.md

### Round 2 redirect (2026-11-25, attempt analysis/redirect/r02-a1) — status: done, recommend (a) small correction round
- Input pass-002 (100.00%, 0 unknown); map/state/source/ROM unchanged. No toolkit change, no publish/archive.
- Whole-map static audit: 0 bad JSR/JMP/branch targets, 0 undefined opcodes in Code, 0 observed fetches outside Code.
- Ready: L-013 (bank 3 $860E 7 B Data->Code; DR_ActionRiderX is 83 entries), L-014 (bank 0 Caves of Fame
  offsets/titles boundaries + $85B1 self-jump), L-015 (commentary IRQ/CLI fact fix), L-016 (provenance from
  multi-player runs), optional L-017 (bank 0 dead fragments) and L-018 (bank 2 $8363 self-modifying fragment).
- Gameplay: S15 multi-player visited (natural menu input; 3 and 6 players). CL_Bracket3, CL_Bracket6 executed; 144 new
  instruction starts, all in Code. S16 (natural full games run) deferred with reason.
- Leads: added L-013..L-018; updated L-004, L-005 (analysis/leads.json). Toolkit: no new issues found.
- Handoff: analysis/handoffs/round-02-redirect.md

### Round 3 disassembly pass (2026-11-25) — status: all bytes classified; overall completion still pending commentary/editorial gates

- Input pass-002 (analysis/state.json restored; edits by tools/r03_apply.py, a guarded pure-JSON edit: only Data/Text
  regions replaced, new Code regions must decode to their exact end, coverage offsets must be decoded Code starts),
  then map.load -> map.validate ok -> map.save analysis/state.json -> map.export maps/current.json. No baseline re-init.
- Output checkpoint: **pass-003** (maps/pass-003.json, generated/pass-003, analysis/pass-003-verification.json:
  ok, byte_identical=true, map_sha256 9008c858...). input/game.nes unchanged (sha256 36a51481...).

ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).

| Range | Bytes | Code | Data | Text | Padding | Image | Unknown |
|---|---|---|---|---|---|---|---|
| Header | 16 | 0 | 16 | 0 | 0 | 0 | 0 |
| PRG16 0 (8K 0/1) menus | 16384 | 3844 | 5878 | 4675 | 1987 | 0 | 0 |
| PRG16 1 (8K 2/3) Mate Toss | 16384 | 4707 | 11397 | 52 | 228 | 0 | 0 |
| PRG16 2 (8K 4/5) Fire Starting | 16384 | 4976 | 7425 | 10 | 3973 | 0 | 0 |
| PRG16 3 (8K 6/7) Dino Race | 16384 | 4833 | 10880 | 128 | 543 | 0 | 0 |
| PRG16 4 (8K 8/9) Saber Race | 16384 | 5614 | 6682 | 0 | 4088 | 0 | 0 |
| PRG16 5 (8K 10/11) Clubbing | 16384 | 7175 | 8990 | 70 | 149 | 0 | 0 |
| PRG16 6 (8K 12/13) Dino Vault | 16384 | 5889 | 8434 | 151 | 1910 | 0 | 0 |
| PRG16 7 (8K 14/15) fixed | 16384 | 4619 | 10304 | 265 | 1196 | 0 | 0 |
| CHR | 131072 | 0 | 0 | 0 | 0 | 131072 | 0 |
| Total | 262160 | 41657 | 70006 | 5351 | 14074 | 131072 | 0 |

- Code provenance: Observed 32555 B (+310), Static 9102 B. 5551 regions.
- Lead outcomes (each re-checked independently; details in analysis/leads.json, analysis/investigation.md):
  - L-013 accepted: bank 3 file 0xC61E-0xC624 ($860E-$8614) Data -> Static Code 0.8 (LDX #1/JSR $8615/LDX #0, falls
    into DR_DrawRacers); DR_ActionRiderX ends at $860D (83 entries).
  - L-014 accepted: bank 0 file 0x557-0x5C3 ($8547-$85B3) re-segmented: Data 0x557-0x55C FameTitleOffsets, Text
    0x55D-0x5C0 FameTitles (5x20), Data 0x5C1-0x5C3 `JMP $85B1` self-jump item.
  - L-016 accepted: 21 Static Code regions with natural multi-player fetches -> Observed (16 whole, 5 split; 310 B).
  - L-017 accepted: bank 0 file 0x12A/0x18A/0x1D3 (8 B each, $811A/$817A/$81C3) Data -> Static Code 0.7.
  - L-018 accepted: bank 2 file 0x8373-0x8382 ($8363) Data -> Static Code 0.7 (self-modifying, non-functional from ROM).
  - L-015 not applied here (commentary-only; left ready for commentary). Rejected: none.
- Verification: map.validate ok; checkpoint pass-003 byte-identical; analysis/redirect/r02-a1/audit.py re-run on the new
  map (scratch/audit_r03.txt): A/B/D/E/G 0, F list lost the 0xC620 hit; 0 observed fetches outside Code.
- Gameplay: no new runs (no unknown bytes; remaining gameplay value is L-004 reachability only).
- Toolkit: no new issues found (analysis/toolkit-issues.md). No processes left running.
- Not complete overall: commentary for the changed ranges and the final editorial audit remain.
- Handoff: analysis/handoffs/round-03-disassembly.md

### Round 3 commentary (2026-11-25) — status: COMPLETE (current-source readability)
- pass-003; round-2 commentary preserved at analysis/round-2/commentary/; commentary/ re-prepared, notes carried by
  physical range. New/changed ranges commented (bank 0 $811A/$817A/$81C3, $8547-$85B3; bank 2 $8363; bank 3
  $85BB-$8615), L-015 IRQ/CLI text fixed (lead -> verified). 857 code units / 5551 rows reviewed; editorial_audit pending.
- Verification: commentary.py verify ok, byte_identical true; reading copy refreshed. No new leads.
- Handoff: analysis/handoffs/round-03-commentary.md

### Round 3 redirect (2026-11-25, attempt analysis/redirect/r03-a1) — status: done, recommend (a) role-correction round
- Input pass-003 (100.00%, 0 unknown); map/state/source/ROM unchanged; static only (no emulator session). No toolkit change.
- New method (not a replay): audit non-code kinds (Padding/Text) and generic-data roles against consumers. The Code graph
  re-audit was clean (audit2.txt).
- Found consumer-anchored data mislabelled as Padding or Text:
  - metatile maps/sets via UploadMetatileScreen (mtile.txt);
  - stride proof: 71 of 71 sixteen-byte "Padding" runs sit on 17-byte record boundaries in banks 3 and 4;
  - direct ADC/LDA reads into Padding;
  - numeric tables mislabelled as Text;
  - the StandingsNameOffsets 6th entry.
- The bank 2 $8A9D block is the FS flame frames (FS_FlameFramePtrs). The commentary's "no reference" was false.
- Bank 5 $AD28-$BEFF and bank 1 $A911-$BD58 have exact consumer-anchored extents.
- Ready: L-019 (125 regions, 1700 Padding + 505 Text bytes -> Data; l019_candidates.json) and L-020 (role/extent
  labels plus commentary fixes). Updated L-004 and L-005. Gameplay: none run; S16 reason unchanged.
- Toolkit: no new issues found. Handoff: analysis/handoffs/round-03-redirect.md

### Round 4 disassembly pass (2026-11-25) — status: all bytes classified; role corrections applied; overall completion still pending commentary/editorial gates

- Input pass-003 (analysis/state.json restored; pre-edit copy analysis/round-4/state.pass-003.json). Edits by
  tools/r04_apply.py (guarded pure-JSON edit: only Data/Text/Padding regions retyped/split/merged, no Code touched,
  full contiguous coverage asserted), then map.load -> map.validate ok -> map.save analysis/state.json -> map.export
  maps/current.json. No baseline re-init. Code regions identical to pass-003 (1614 regions, 41657 B).
- Output checkpoint: **pass-004** (maps/pass-004.json == maps/current.json, generated/pass-004,
  analysis/pass-004-verification.json: ok, byte_identical=true, map_sha256 84eebb55...). input/game.nes unchanged
  (sha256 36a51481...).

ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).

| Range | Bytes | Code | Data | Text | Padding | Image | Unknown |
|---|---|---|---|---|---|---|---|
| Header | 16 | 0 | 16 | 0 | 0 | 0 | 0 |
| PRG16 0 (8K 0/1) menus | 16384 | 3844 | 6289 | 4560 | 1691 | 0 | 0 |
| PRG16 1 (8K 2/3) Mate Toss | 16384 | 4707 | 11465 | 10 | 202 | 0 | 0 |
| PRG16 2 (8K 4/5) Fire Starting | 16384 | 4976 | 7435 | 0 | 3973 | 0 | 0 |
| PRG16 3 (8K 6/7) Dino Race | 16384 | 4833 | 11418 | 0 | 133 | 0 | 0 |
| PRG16 4 (8K 8/9) Saber Race | 16384 | 5614 | 7634 | 0 | 3136 | 0 | 0 |
| PRG16 5 (8K 10/11) Clubbing | 16384 | 7175 | 9040 | 36 | 133 | 0 | 0 |
| PRG16 6 (8K 12/13) Dino Vault | 16384 | 5889 | 8585 | 0 | 1910 | 0 | 0 |
| PRG16 7 (8K 14/15) fixed | 16384 | 4619 | 10313 | 256 | 1196 | 0 | 0 |
| CHR | 131072 | 0 | 0 | 0 | 0 | 131072 | 0 |
| Total | 262160 | 41657 | 72195 | 4862 | 12374 | 131072 | 0 |

- Regions 5551 -> 4306 (fragment merges). Code provenance unchanged: Observed 32555 B, Static 9102 B.
- Lead outcomes (independent re-check: tools/r04_verify.py -> analysis/round-4/l019_verify.json, extents.txt):
  - L-019 accepted (125/125 candidates; 2189 B: Padding 1700 -> Data, Text 489 -> Data). Metatile map/set membership
    re-derived by a different method (straight-line data-flow pairing of MtileMapPtr/MtileSetPtr immediates at each
    JSR/JMP UploadMetatileScreen, not r03's nearest-window pairing); cited direct consumers re-decoded; DR pose tables
    (4 x 61 B parallel), DV_CavemanDX (173 B parallel to DV_CavemanDY), StandingsNameOffsets, MT frame membership checked
    by hand. Bank 3 records 76..103 of set $9F23 accepted on stride evidence (every record 65..103 starts with $55; 23/23
    zero runs at set+17k+1), conf 0.8, natural use unresolved. Bank 0 $A3E5/$A3FA letter-tile records accepted as Data.
    Bank 1 $9D79/$9D92 accepted as generic Data (not prose; consumer unidentified). Bank 7 $D257: split, only the 1-byte
    6th offset entry -> Data; $D258-$D267 "        RESULTS." stays Text. Bank 3 $8599+42 split at $85BB (RiderY/RiderX).
    Partial bank-0 regions split at $A593/$A9D9/$AAE4; leading zero runs ($A581+18, $A9AC+45, $AAC7+29) stay Padding.
    Bank 7 $D899 left Text (unanchored, as recommended).
  - L-020 accepted (disassembly part): labelled Data blocks FS_FlameCels (bank 2 $8A9D-$8D00), FS_PoseCels ($93A0-$941F),
    pointer tables $9358-$9379 and $937A-$939F split out; CL_ArenaMap/CL_ArenaMetatiles/CL_MsprCels (bank 5
    $AD28/$AD68/$AF00-$BEFF); MT_MsprCels (bank 1 $A911-$BD58) plus pointer table $A8AD-$A910; plus 16 verified metatile
    map/set extents merged into single Data regions (banks 0,1,2,3,4,6,7). Commentary parts of L-020 left for commentary.
    Not merged: bank 3 course map $944B-$9560 (exact extent unproven), bank 6 map $80E6 (set in effect comes from caller),
    bank 2 $9420-$944B (no consumer).
  - Rejected: none.
- Verification: map.validate ok; checkpoint pass-004 byte-identical; Code region set identical to pass-003.
- Gameplay: no new runs (no unknown bytes; static consumer evidence sufficed; scenarios.json unchanged).
- Toolkit: no new issues found (analysis/toolkit-issues.md). No processes left running.
- Handoff: analysis/handoffs/round-04-disassembly.md

### Round 4 commentary (2026-11-25) — status: COMPLETE (current-source readability)
- pass-004; round-3 commentary preserved at analysis/round-3/commentary/ (diff -r identical); commentary/ re-prepared
  (4306 rows), notes/tools/uncertainties carried by physical range (all notes accepted by the instruction-start check).
- L-020 commentary part done: frame/pointer tables and metatile maps/sets named and explained in banks 0-7; banked
  SFX tables in banks 1-6 re-cut per bank; stale "no reference"/"generic data after frames"/padding/text prose and
  stray propagation headers removed; bank 5 $8074 and bank 6 $AD22 operand notes. L-020 -> verified; new lead L-021.
- 857 code units (829 identical, 28 header/extent text refreshed) + 189 data units, all reviewed; 4306 rows reviewed;
  editorial_audit pending. Verification: commentary.py verify ok, byte_identical true; reading copy refreshed.
- Handoff: analysis/handoffs/round-04-commentary.md

### Round 4 redirect (2026-11-25, attempt analysis/redirect/r04-a1) — status: done, recommend (a) short round (L-021, L-022) then final acceptance
- Static only; pass-004 (100.00%, 0 unknown); map/state/source/ROM/toolkit unchanged; no processes left running.
- Role-unknown blocks re-checked: no consumer, no code evidence (consumers.txt, codeprobe.txt); bank 3 $B375-$BEFF
  shows an address-aligned 8-low/8-high popcount pattern (period.txt), so it is non-code.
- New L-022: DR_CourseMap exact extent $944B-$954A (256 B) from the fixed pointer, the 8-bit (ind),Y index and the finish look-ahead $FF;
  $954B-$9560 (22 B) outside the consumer window. L-021 confirmed (+ optional bank 2 $ACE0 split, commentary notes).
- Leads: added L-022; updated L-021, L-004, L-005. Ready: L-021, L-022. Handoff analysis/handoffs/round-04-redirect.md

### Round 5 disassembly pass (2026-11-25) — status: all bytes classified; L-021/L-022 applied; overall completion still pending commentary/editorial gates

- Input pass-004 (analysis/state.json restored; pre-edit copy analysis/round-5/state.pass-004.json). Edits by
  tools/r05_apply.py (guarded pure-JSON edit of Data/Text regions only, byte-pattern asserts, contiguous coverage asserted)
  -> map.load -> map.validate ok -> map.save analysis/state.json -> map.export maps/current.json. No baseline re-init.
  Code region set identical to pass-004 (1614 regions, 41657 B; provenance unchanged).
- Output checkpoint: **pass-005** (maps/pass-005.json == maps/current.json, generated/pass-005,
  analysis/pass-005-verification.json: ok, byte_identical=true, map_sha256 b25ddb4d...). input/game.nes unchanged (sha256 36a51481...).

ROM classification: 100.00% (262160 / 262160 bytes classified; 0 bytes unknown).

| Range | Bytes | Code | Data | Text | Padding | Image | Unknown |
|---|---|---|---|---|---|---|---|
| Header | 16 | 0 | 16 | 0 | 0 | 0 | 0 |
| PRG16 0 (8K 0/1) menus | 16384 | 3844 | 6318 | 4531 | 1691 | 0 | 0 |
| PRG16 1 (8K 2/3) Mate Toss | 16384 | 4707 | 11465 | 10 | 202 | 0 | 0 |
| PRG16 2 (8K 4/5) Fire Starting | 16384 | 4976 | 7435 | 0 | 3973 | 0 | 0 |
| PRG16 3 (8K 6/7) Dino Race | 16384 | 4833 | 11418 | 0 | 133 | 0 | 0 |
| PRG16 4 (8K 8/9) Saber Race | 16384 | 5614 | 7634 | 0 | 3136 | 0 | 0 |
| PRG16 5 (8K 10/11) Clubbing | 16384 | 7175 | 9040 | 36 | 133 | 0 | 0 |
| PRG16 6 (8K 12/13) Dino Vault | 16384 | 5889 | 8585 | 0 | 1910 | 0 | 0 |
| PRG16 7 (8K 14/15) fixed | 16384 | 4619 | 10324 | 245 | 1196 | 0 | 0 |
| CHR | 131072 | 0 | 0 | 0 | 0 | 131072 | 0 |
| Total | 262160 | 41657 | 72235 | 4822 | 12374 | 131072 | 0 |

- Regions 4306 -> 4280. Changed ranges (analysis/round-5/changes.json):
  - bank 7 $D881-$D922 (file 0x1D891-0x1D932, 162 B): 13 pieces -> one Data region (MenuMetatiles records 41-49 + 9 zero
    bytes); Text $D899-$D8A3 (11 B) -> Data.
  - bank 0 $A8CF-$A9AB (0x28DF-0x29BB, 221 B): 16 pieces -> one Data region ($FF-separated tile-number lists); Text
    $A92E-$A94A (29 B) -> Data.
  - bank 2: boundary moved from $ACDF to $ACE0 (0xACF0): $ACDF (SFX stream 17 end byte) joins $AC4F-$ACDF; $ACE0-$AFFF fill.
  - bank 3 $944B-$954A (0xD45B-0xD55A, 256 B) Data label DR_CourseMap (Static 0.9); $954B-$9560 (0xD55B-0xD570, 22 B) generic Data.
- Lead outcomes:
  - L-021 verified/accepted (incl. optional bank 2 split). Independent re-check: attribute bytes at $D881+17k all $05/$55,
    9 trailing zeros; bank 0 run is a continuation of ascending tile numbers between $FF separators; stream 17 pointer
    ($AC76 -> $ACDB) bytes 81 78 0E 01 80.
  - L-022 verified/accepted. Independent re-check: raw scan finds only two bank-3 MtileMapPtr writers ($81AC -> $944B,
    $9421 -> $9561), none in bank 7; the sole read is LDA ($A4),Y in MetatilePtrFromMap with 8-bit Y (window <= $954A);
    finish look-ahead $FF reused from r04-a1 coursemap.txt.
  - Rejected: none. Commentary follow-ups: replace "end is not proven" for DR_CourseMap, refresh notes for the merged
    bank 0/7 regions (commentary/ not touched here).
- Gameplay: no new runs (static evidence sufficed; scenarios.json unchanged).
- Toolkit: no new issues found (analysis/toolkit-issues.md). No processes left running.
- Handoff: analysis/handoffs/round-05-disassembly.md

### Round 5 commentary (2026-11-25) — status: COMPLETE (current-source readability)
- pass-005; round-4 commentary preserved at analysis/round-4/commentary/ (diff -r identical); commentary/ re-prepared
  (4280 rows); notes/tools/uncertainties carried by physical range (all accepted).
- DR_CourseMap 256-B proof in source (L-022), $954B-$9560 tail note; bank 0 $A8CF and bank 7 $D881 notes no longer
  treat the merged ranges as Text (L-021); weak hints for bank 4 $B348 / bank 2 $9420 as uncertain notes;
  uncertainties.md Round 5 section. update_review.py data-unit extents tightened.
- 857 code + 192 data units, all reviewed; 4280 rows reviewed; editorial_audit pending. verify ok, byte_identical true;
  reading copy refreshed. Leads: L-021/L-022 commentary attempts recorded (verified), L-005 hint note (deferred).
- Handoff: analysis/handoffs/round-05-commentary.md

### Final editorial acceptance (2026-11-25) — status: ACCEPTED (audit ok, byte-identical)
- pass-005; targeted checks (raw operands, auto labels, stale prose, unit/label/map boundaries, name collisions,
  certainty wording). Fixed:
  - stale numeric aliases: reset stubs -> InitRecords/WarmStart, CL_BracketHandlers-2/-1, DV_OutcomeTable-12,
    FS_UnusedEmphasisCycle+1, Var04AD -> DvVaultPhase
  - 21 code-unit extents over trailing data
  - 847 split continuation records (first audit failed: region-6 lacked units)
  - two over-certain "unused" claims
- editorial_audit reviewed. `commentary.py audit` ok: 4280 regions + 1896 unit records, byte_identical true.
  Artifacts: commentary/audit.json and commentary/reading-after.asm. Pre-edit snapshot in analysis/final-acceptance/pre-edit/.
- Remaining explained uncertainties: generic data without consumers, L-004 reachability, L-005/L-006, IrqHandler.
- No toolkit issues. Handoff: analysis/handoffs/final-acceptance.md
