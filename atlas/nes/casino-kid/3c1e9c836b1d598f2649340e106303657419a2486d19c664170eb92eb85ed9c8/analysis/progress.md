# Disassembly progress

Workspace initialized. Baseline and classification had not run before round 1.

## Coordinator terminal handoff (2026-11-24)

Workflow state: TERMINAL — documented whole-ROM-complete branch reached; ready for archive
(archive NOT executed; no cleanup performed).
- Final ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown),
  checkpoint pass-003 (maps/pass-003.json == maps/current.json == analysis/state.json).
- Evidence-backed classification per round-2 disassembly; rejection review in
  analysis/handoffs/round-2-redirect.md reviewed by coordinator and accepted (empty unknown
  complement; residuals L4-L7 are reachability / data-semantics only; 5624 code bytes remain
  Static provenance, not Observed).
- Commentary: full readability gate met; final editorial audit passed
  (commentary/editorial-audit.md, commentary/audit.json); verification byte-identical.
- Known gaps: analysis/annotations.json not produced (no NES tool; TK-005);
  analysis/current-audit.json is a summary assembled from tool outputs (see its generated_by).
- Archive document selection: archive-options.json (all analysis/commentary Markdown accounted for).
- This orchestration ending is not itself the basis of the completion claim; see the
  verification artifacts above.

## Coordinator log (goal.md orchestration)

Coordinator keeps only round/phase summaries here; phase workers append their own
detailed sections and handoffs (analysis/handoffs/round-N-<phase>.md).

| Round | Phase | Status | Input checkpoint | Output checkpoint | Handoff |
|---|---|---|---|---|---|
| 1 | disassembly | done: 100.00% (131088/131088; 0 unknown), verified byte-identical | (none; baseline created) | pass-002 | analysis/handoffs/round-1-disassembly.md |
| 1 | commentary | done: full readability gate met (518 named routines, 0 uncovered spans; final editorial audit pending), commentary verify byte-identical | pass-002 | pass-002 (no map change) | analysis/handoffs/round-1-commentary.md |
| 1 | redirect | done (attempt r1-a1): blackjack reached; ready leads L8 ($CFF6 data->code), L2 (coverage merge), L13, L14; commentary fixes L9-L12 | pass-002 | pass-002 (unchanged) | analysis/handoffs/round-1-redirect.md |
| 2 | disassembly | done: 100.00% (131088/131088; 0 unknown), verified byte-identical; L8, L2, L13, L14 verified | pass-002 | pass-003 | analysis/handoffs/round-2-disassembly.md |
| 2 | commentary | done: full readability gate met (1582 regions reviewed, 0 mechanical labels, L9-L14 applied; editorial audit pending), verify byte-identical; produced L15 | pass-003 | pass-003 (no map change) | analysis/handoffs/round-2-commentary.md |
| 2 | redirect | done (attempt r2-a1): no structural leads; L15 commentary-only; rejection review in handoff, coordinator-reviewed and accepted (empty unknown complement; residuals are reachability/provenance/semantic-label only) | pass-003 | pass-003 (unchanged) | analysis/handoffs/round-2-redirect.md |
| final | acceptance | done: L15 applied; commentary.py audit passed (1582 regions / 1435 units reviewed, byte-identical); maps/state validate ok; pass-003 regenerate+verify byte-identical | pass-003 | pass-003 (no map change) | analysis/handoffs/final-acceptance.md |

## Round 1 disassembly (2026-11-23)

Input: no baseline, empty map. Baseline created (`python3 run.py baseline`, maps/baseline.json,
generated/baseline, analysis/baseline-verification.json: ok, byte_identical).

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

Per bank (physical file; header = 16 bytes; bank N = 16 KiB PRG bank N):

| Bank | Code (Observed / Static) | Data | Unknown |
|---|---|---|---|
| header | 0 | 16 | 0 |
| 0 | 3 (0 / 3) | 16381 | 0 |
| 1 | 7 (0 / 7) | 16377 | 0 |
| 2 | 7 (0 / 7) | 16377 | 0 |
| 3 | 3807 (0 / 3807) | 12577 | 0 |
| 4 | 6917 (3795 / 3122) | 9467 | 0 |
| 5 | 5956 (2602 / 3354) | 10428 | 0 |
| 6 | 4888 (3804 / 1084) | 11496 | 0 |
| 7 (fixed) | 10239 (6581 / 3658) | 6145 | 0 |
| total | 31824 (16782 / 15042) | 99264 | 0 |

Regions: 1493 (429 Code/Observed, 496 Code/Static, 568 Data/Static). Execution coverage
(separate metric): 8250 observed opcode starts (traces/coverage.json, 6 traced scenarios).
All data is generic or simple structural (header, vectors, vector copies, inline jump tables,
inline parameter bytes, ASCII title, zero runs); semantic labels are mostly absent.

Checkpoints: pass-001 (static + boot/casino coverage) and pass-002 (final, adds overworld,
password, KING debug path, poker, ending coverage). Verification: maps/pass-002.json
(sha256 c3ae9a83...cd99) -> generated/pass-002, analysis/pass-002-verification.json
ok=true, byte_identical=true. Blind check: all 8250 observed opcode starts from the second
replay fell inside pass-001 Code regions at instruction starts (0 conflicts).

Evidence/method: analysis/investigation.md (Round 1). Scenarios: analysis/scenarios.json
(S1-S7 visited; S5-S7 artificial via RAM patches; S8 blackjack deferred). Leads: none consumed
(none existed); produced L1-L7 in analysis/leads.json. Toolkit: analysis/toolkit-issues.md
(1 confirmed bug TK-001, 2 missing features TK-002/TK-003; no blockers).

Status: pass finished; ROM NOT declared complete (commentary/editorial gates pending; static
code in bank 3 and parts of 4-7 unconfirmed by execution; generic data semantics unknown).
Handoff: analysis/handoffs/round-1-disassembly.md

## Round 1 commentary (2026-11-24)

Input checkpoint: pass-002 (maps/current.json == maps/pass-002.json; commentary prepared
fresh from it with `commentary.py prepare`, frozen baseline commentary/before/, byte-identical).
Output: commentary/source (game.asm + ram.inc, built by tools/cmt.py from commentary/notes/*.ann).
Verification: `commentary.py verify` ok, byte_identical=true (commentary/verification.json);
reading copy refreshed (commentary/reading-after.asm). Map/classification unchanged
(ROM classification still 100.00%; no structural edits).

Readability scope: reviewed = all classified source - 925 code regions (31,824 bytes, banks 0-7)
and 568 data regions. Every code span has a routine block comment (`tools/cmt.py todo` = 0),
no address-only routine names remain (518 global labels named), ~230 RAM/constant symbols,
inline parameters and jump tables rendered symbolically. Remaining: none for the round-1 gate;
improvement backlog = ~960 mechanical local branch labels (@loop/@skip/@case), data record
formats (L5/L6), runtime confirmation of bank 3/4/5 inferences. review.json: 1493/1493 entries
reviewed (1140 units); editorial_audit pending (final acceptance only).

Leads consumed: none. Produced: L8-cff6-remnant, L9-poker-ace-high-straight,
L10-blackjack-semantics, L11-debug-screen, L12-broke-screen-entry, L13-preset-passwords.
Handoff: analysis/handoffs/round-1-commentary.md

## Round 1 redirect (2026-11-24, attempt r1-a1)

Input checkpoint: pass-002 (maps/current.json unchanged; commentary round-1 source).
Research directory: analysis/redirect/r1-a1/ (notes.md, tools, logs, coverage-all.json,
snapshots bjbet.json/pkbet.json). ROM, map, state, source and binary unchanged.
ROM classification unchanged: 100.00% (131088/131088; 0 unknown); exact unknown complement empty.

Method: static decoding of the floor talk/encounter path, then bounded artificial-state runs
(position patch next to a seat/NPC, deck patches, RAM-written password buffers) with natural
game logic afterwards; each scenario prototyped untraced and replayed traced with identical
inputs; whole-map boundary audit (targetaudit.py), inline-contract audit (inlinecheck.py),
operand/extent tiling of data tables (refs.py).

Outcomes:
- Blackjack reached and played (13 rounds, dealer broke, progress recorded); poker vs a
  regular gambler; debug monitor; GAME OVER; password encode/decode round trip.
- Execution coverage (separate metric): +4438 opcode starts (bank 3 1659/1862 static starts);
  0 conflicts with map instruction boundaries or Code/Data kinds. Under the mkmap full-region
  rule 5219 Static code bytes would become Observed (provdelta.txt).
- 276 inline-callee call sites confirmed; jump-table words all hit Code starts.
- One structural proposal: $CFF6 (1 byte) Data -> Code (L8). Label/structure proposals for
  already-classified data (L13 passwords, L14 bank 3/4 tables). Commentary corrections L9-L12.

Leads consumed: L1, L3, L9, L11, L12, L13 (L1/L3 verified). Updated: L2, L4-L8, L10.
Produced: L14-bank34-game-data-tables. Ready for next disassembly: L8, L2, L13, L14; for next
commentary: L9, L10, L11, L12, L13. Toolkit: TK-004 added (trace aggregation cost).
Handoff: analysis/handoffs/round-1-redirect.md

## Round 2 disassembly (2026-11-24)

Input checkpoint: pass-002 (state restored; analysis/state.json == maps/pass-002.json; mkmap.py
reproduced it exactly before edits). Output checkpoint: pass-003 (maps/pass-003.json sha256
9e2a7bca...22f2 -> generated/pass-003; analysis/pass-003-verification.json ok=true,
byte_identical=true). map.load + map.validate ok; analysis/state.json == maps/current.json.

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

| Bank | Code (Observed / Static) | Data | Unknown |
|---|---|---|---|
| header | 0 | 16 | 0 |
| 0 | 3 (0 / 3) | 16381 | 0 |
| 1 | 7 (0 / 7) | 16377 | 0 |
| 2 | 7 (0 / 7) | 16377 | 0 |
| 3 | 3807 (3389 / 418) | 12577 | 0 |
| 4 | 6917 (4456 / 2461) | 9467 | 0 |
| 5 | 5956 (5354 / 602) | 10428 | 0 |
| 6 | 4888 (4207 / 681) | 11496 | 0 |
| 7 (fixed) | 10240 (8795 / 1445) | 6144 | 0 |
| total | 31825 (26201 / 5624) | 99263 | 0 |

Regions: 1582 (627 Code/Observed, 372 Code/Static, 583 Data/Static); 39 labels (21 new data
labels). Execution coverage (separate metric): 12688 observed opcode starts in traces/coverage.json
(round 1 8250 + redirect r1-a1 4438).

Changes (all through tools/mkmap.py -> scratch/newmap.json -> map.load):
- L8 verified: physical 118790 ($CFF6, bank 7) Data -> Code/Static (PHA; target of the
  fragment's JMP/BNE; falls into established $CFF7). Only instruction-start change vs pass-002.
  Reachability unresolved.
- L2 verified: redirect coverage merged (tag redirect_r1a1). 0 of 11050 session opcode starts off
  pass-002 instruction starts. 9419 code bytes Static -> Observed (per-instruction runs;
  bank 3 3389, 4 661, 5 2752, 6 403, 7 2214). Some sessions used recorded RAM position/password
  patches (artificial entry; execution observed, natural reachability separate).
- L13 verified: cheat password strings bank 5 (4 x 30 at $93DF) and bank 6 PresetPasswordPtrs +
  4 x 30 strings, labelled; wrong "no operand reference" rationale removed.
- L14 verified: bank 3 $913A-$9B87 and bank 4 $9F06-$BABF split into consumer-anchored tables
  (labels Bj*/Pk*); consumer-unidentified text spans stay unlabelled Data. Correction:
  PkHandNameStrings ends at $A408 (not $A405).
Gameplay: no new play. All milestones incl. blackjack, King and ending are visited
(analysis/scenarios.json S1-S13); remaining static code (5624 bytes) only affects provenance and
traced play costs ~10x (TK-004), so further play was judged not a substantial gain.
Leads consumed: L8, L2, L13, L14 (all verified). Produced: none. Ready for commentary: L9-L14.
Toolkit: no new toolkit issues found this pass (existing TK-001..TK-004 in analysis/toolkit-issues.md).
Status: pass finished; classification complete by count, overall ROM NOT declared complete
(commentary must absorb region changes; editorial audit pending).
Handoff: analysis/handoffs/round-2-disassembly.md

## Round 2 commentary (2026-11-24)

The commentary was rebuilt on checkpoint pass-003. The round-1 commentary is preserved in
analysis/round-1/. commentary/ was re-prepared from pass-003, and the round-1 annotation specs
were carried over by address.

- Readability gate: met for all currently classified source.
  - Every code region is covered by routine comments (cmt.py todo: 0 uncommented spans).
  - 0 mechanical branch labels remain (966 renamed).
  - 0 raw RAM operands remain.
  - The only raw ROM operands left are the commented cross-bank entry slots.
- Corrections applied: L9-L13 (L14 labels adopted). Round-1 wrong notes fixed:
  - the cursor-init tables were wrongly called "unused";
  - PasswordMoneyLimit;
  - the size of BjDealerName.
- Verification: commentary.py verify is ok and byte-identical. The reading copy is refreshed.
  - review.json: 1582/1582 entries reviewed, with 1419 units.
  - The editorial audit is pending; it is reserved for final acceptance.
- Classification is unchanged: 100.00% (pass-003). This handoff does not establish whole-ROM
  completion.
- Leads: L9-L14 commentary items applied. Produced L15-residual-field-semantics (low priority,
  commentary certainty only).
- Toolkit: no new issues.
- Handoff: analysis/handoffs/round-2-commentary.md

## Round 2 redirect (2026-11-24)

Attempt r2-a1, input checkpoint pass-003. The map is unchanged: 100.00% classified, 0 unknown
bytes. Only research and lead records changed. There were no runtime sessions.
- L15 was resolved statically (analysis/redirect/r2-a1/notes.md):
  - Password packing: three 25-bit amounts (bits 2..26) are packed without overlap at
    PasswordBytes bits 0-24, 25-49 and 50-74.
  - PkOpponentTriples +0 and BjDealerRecordTable +10 are constant fields with no reader.
  - Blackjack result codes 1, 3 and 8 have explicit static writers.
- L15 is now ready, as a commentary-only lead for the final editorial audit.
- L4-L7: reviewed, with no new evidence. They are reachability or semantic-label questions
  only.
- Ready structural leads: none. The rejection review is in analysis/handoffs/round-2-redirect.md.
- Recommendation: terminal for classification. Next comes final acceptance: apply L15 in
  commentary, run the editorial audit, then final verification.
- Toolkit: no new issues.
- Handoff: analysis/handoffs/round-2-redirect.md

## Final acceptance (2026-11-24)

Checkpoint: pass-003. maps/current.json, maps/pass-003.json and analysis/state.json share
sha256 9e2a7bca...22f2.

ROM classification is unchanged: 100.00% (131088 / 131088 bytes classified; 0 unknown).
That is 1582 regions: Code 31825 bytes and Data 99263 bytes.

Work done:
- Applied lead L15 as commentary wording. It is now marked applied in leads.json.
- Ran the final editorial audit (commentary/editorial-audit.md).
- `commentary.py audit` passed (commentary/audit.json):
  - 1582 regions and 1435 units reviewed;
  - byte-identical rebuild;
  - reading copy refreshed.

Verification with the configured binary:
- `map validate` returned ok for maps/current.json and maps/pass-003.json.
- `store validate` returned ok for analysis/state.json.
- generate + verify of pass-003 into scratch/final-acceptance/pass-003-gen returned
  byte_identical=true.
- Results are summarised in analysis/current-audit.json.

Remaining items:
- analysis/annotations.json has no producing tool (TK-005). It was not fabricated.
- The open leads L4-L7 are reachability or semantic-label questions only.

No archive, publication or toolkit change was made. The ROM is unchanged.

Handoff: analysis/handoffs/final-acceptance.md
