# Disassembly progress

Workspace initialized. Baseline and classification have not run.

## Coordinator state

- Round: 1 (not started)
- Current phase: disassembly (pending; prompts/disassembly.txt not yet executed)
- Input checkpoint: none (no baseline yet)
- Output checkpoint: none
- Leads consumed/produced: none
- ROM classification completion: not measured (no baseline/state yet)

## 2026-11-14 coordinator session — BLOCKED (environment)

Every Bash invocation fails before the command runs, both in the coordinator
session and in a spawned subagent (checked with a trivial `echo` worker):

    apply-seccomp: write /proc/self/setgroups (nested userns is capability-restricted; caller must provide CAP_SYS_ADMIN): Permission denied

The workflow needs command execution (`python3 run.py inspect/baseline/serve/checkpoint`
and the configured binary /root/romatlas/target/release/romatlas). Without it, no
phase worker can run baseline, discovery, the emulator or verification. This is a
host sandbox/runner configuration problem (bubblewrap cannot create a nested user
namespace), not a toolkit or ROM issue, so it is not recorded as a toolkit finding.

Nothing was modified apart from this progress note. No baseline, state, map or
checkpoint was created. To resume, run the runner on a host where the Bash sandbox
works (or where the sandbox is configured so it can start), then begin round 1 at the
disassembly phase (workflow.md step 1: `python3 run.py inspect`, then `python3 run.py baseline`).
archive-options.json is deliberately not written yet: no terminal handoff exists,
and analysis/ holds only the initial template documents.

## 2026-11-18 coordinator session — environment unblocked

Bash execution works again (sandbox issue resolved). Starting round 1.

- Round 1, phase: disassembly — STARTED (worker launched; prompt prompts/disassembly.txt)
- Input checkpoint: none (baseline to be created by this worker)

## Round 1 disassembly handoff (2026-11-18)

- Scope: one easy-gain disassembly pass from a fresh workspace. Baseline created
  (`python3 run.py baseline`, analysis/baseline-verification.json: ok, byte_identical).
  Static bank-aware discovery plus traced gameplay, then map.annotate of 3,052 regions.
- Checkpoint: **pass-001** (maps/pass-001.json, generated/pass-001,
  analysis/pass-001-verification.json). Verification: ok=true, byte_identical=true;
  map sha256 8f3227e7962589c5a146e2d587d12cd0b8923491939dc54ca5ea9d9fc173144d.
  map.validate ok; state saved to analysis/state.json; maps/current.json exported.
- Code 57,337 B (Observed 34,437 / Static 22,900); Data 157,012; Padding 24,524;
  Image 131,072 (CHR); header 16 B Data. Execution coverage: 17,278 distinct ROM
  instruction starts observed. This is a separate metric from classification.
- Per-bank classified/unknown (PRG banks of 8 KiB; code / data+padding / unknown):

| bank | code | data | unknown |
|---|---|---|---|
| 0 | 1669 | 5645 | 878 |
| 1 | 0 | 7418 | 774 |
| 2 | 1669 | 6020 | 503 |
| 3 | 0 | 7564 | 628 |
| 4 | 1669 | 5876 | 647 |
| 5 | 0 | 7204 | 988 |
| 6 | 1669 | 5944 | 579 |
| 7 | 0 | 7204 | 988 |
| 8 | 1167 | 6572 | 453 |
| 9 | 0 | 7590 | 602 |
| 10 | 155 | 7033 | 1004 |
| 11 | 841 | 7042 | 309 |
| 12 | 155 | 6769 | 1268 |
| 13 | 700 | 6964 | 528 |
| 14 | 155 | 7027 | 1010 |
| 15 | 356 | 7049 | 787 |
| 16 | 2749 | 4701 | 742 |
| 17 | 3918 | 3657 | 617 |
| 18 | 155 | 6811 | 1226 |
| 19 | 359 | 7363 | 470 |
| 20 | 6838 | 568 | 786 |
| 21 | 1518 | 5947 | 727 |
| 22 | 4124 | 2154 | 1914 |
| 23 | 2868 | 4951 | 373 |
| 24 | 494 | 7115 | 583 |
| 25 | 48 | 7593 | 551 |
| 26 | 7536 | 430 | 226 |
| 27 | 1633 | 4340 | 2219 |
| 28 | 6638 | 1378 | 176 |
| 29 | 2606 | 5178 | 408 |
| 30 | 5084 | 2810 | 298 |
| 31 | 564 | 7603 | 25 |
| header | 0 | 16 | 0 |
| CHR (16x8 KiB) | 0 | 131072 | 0 |

- Evidence/method: analysis/investigation.md. Scripts: analysis/round1-tools/.
  Scenarios: analysis/scenarios.json (S01-S08 visited; S09 season end/ending and
  S10 2P deferred). Gameplay notes: analysis/gameplay.md.
- Scenarios exercised: title; menus; Exhibition WATCH full CPU game (traced to frame
  ~5.7k, then untraced); game-end via RAM patch $037B<-$0F (artificial); password entry
  and invalid password; Regular Season START, game-1 intro, batting-order edit, human
  pitching/fielding; Options toggles; Exhibition 1P human batting. Coverage growth fell
  to ~15 new instructions per 650-frame input cycle, which marks easy-gain exhaustion.
- Leads produced (analysis/leads.json): L001 non-closed inline jump tables; L002
  residual coverable unknown runs (~23 KiB); L003 unit 28 entry 7 data pointer; L004
  unreferenced bank 30 gaps; L005 season/ending RAM shortcut; L006 bank 31 residues;
  L007 units 0/2/4/6 shared code reachability; L008 (deferred) manual fielding stall.
  No leads were consumed (none existed).
- Unresolved questions: consumers/layouts of the generic data; natural reachability of
  22.9 KiB static-only code; season RAM contract and ending.
- Deferred work: season-end/ending (L005), 2P mode, ROM data-read harvesting (Read
  events include fetches; needs coverage subtraction).
- Toolkit findings (analysis/toolkit-issues.md): TK-001 confirmed bug (NES
  emulator.input ignores documented button/pressed; workaround `buttons` array),
  TK-002 missing feature (NES static discovery stops at every JSR/indexed store on
  MMC3; `budget` vs `max_instructions`). Neither is blocking.
- Status: incomplete (23,287 bytes unknown).

ROM classification: 94.08% (369945 / 393232 bytes classified; 23287 bytes unknown).

## Coordinator: round 1 phase transition

- Round 1 disassembly: COMPLETE (easy-gain exhaustion). Output checkpoint pass-001
  (verified ok, byte_identical). Leads produced L001-L008. Reported to user:
  ROM classification: 94.08% (369945 / 393232 bytes classified; 23287 bytes unknown).
- Round 1, phase: commentary — STARTED. Input checkpoint pass-001.

## 2026-11-18 coordinator resume

- Previous commentary worker was interrupted right after preparation (review.json all
  4143 entries pending; only commentary/tools/model.py, xref.py helper scripts created).
- Round 1, phase: commentary — RESUMED (worker 1b). Input checkpoint pass-001.

## Commentary round 1 (2026-11-18) - INCOMPLETE
- Generated commented source (commentary/tools/annotate.py + knowledge files), verified
  byte-identical; reading copy refreshed. Fixed bank, sound driver, field streamer, mode
  entries and key helpers hand-documented; 905 unit code entries still pending semantic
  naming (ranges in commentary/progress.md). Handoff: analysis/commentary-handoff-round1.md.
- Round 1 commentary continuation: worker 1c assigned unit 28 (then sequential workers per remaining unit group).
- Commentary round 1 worker 1c: unit 28 fully hand-named/reviewed (368 units); unit 26
  $8007-$95AF reviewed (162 units), 112 pending in 0x355C0-0x36768. verify ok
  (byte-identical); reading copy refreshed. 805 code units still pending overall.
- Worker 1d assigned: unit 26 remainder (0x355C0-0x36768) then unit 22.
- Commentary round 1 worker 2: unit 26 remainder ($95B0-$A757: dive states, fence collision,
  fielder AI/assignments) and all of unit 22 (runners, umpire, CPU input service) hand-named
  and reviewed; 436 code units still pending (units 20/16/12/24/14/18/10). verify ok
  (byte-identical); reading copy refreshed.
- Worker 1e assigned: unit 20 (0x28021-0x2A6F1), then unit 16 if context allows.

- Commentary round 1 worker 3: units 20 (GameMode 3 front end, all 21 sub-modes identified) and 16
  (at-bat panel, cut-scenes, password encode/decode/editor) fully named and reviewed in
  commentary/tools/knowledge_u20.py / knowledge_u16.py; verify byte-identical. Remaining: units 24/12/14/18/10.
- Worker 1f assigned: units 24/12/14/18/10, then whole-source naming gap sweep and commentary handoff.

- Commentary round 1 worker 1f: units 24/12/14/18/10 named and reviewed (knowledge_rest.py);
  readability sweep: all raw RAM operands named, pitcher-condition renames, generic labels in
  fixed bank/unit 8 named. review.json: no pending entries/units (editorial_audit pending).
  verify ok (byte-identical); reading copy refreshed. Commentary readability gate met for
  pass-001; handoff analysis/commentary-handoff-round1.md (final audit not run).

## Coordinator: round 1 commentary -> redirect

- Round 1 commentary: COMPLETE (readability gate). review.json 4142 entries reviewed,
  1728 code units reviewed, editorial_audit pending (final acceptance). Coordinator re-ran
  commentary.py verify: ok, byte_identical. Handoff analysis/commentary-handoff-round1.md.
  Residual: ~1600 generic @Loop_/@Skip_ branch labels (final editorial pass).
- Round 1, phase: redirect — STARTED (worker 1g). Input checkpoint pass-001 + commentary round 1.

## Round 1 redirect (worker 1g, attempt r1-a) - COMPLETE, ready leads produced

- Input checkpoint pass-001 + commentary round 1. Handoff: analysis/redirect/r1-a/handoff.md;
  exact unknown ledger analysis/redirect/r1-a/unknown-ledger.json (all 23,287 unknown bytes
  given a disposition). No map/state/source changes.
- Consumed: L001, L002, L004, L006. Updated: L003 (closed, no change), L005 (progressed:
  pennant, Super Series, SUPER SERIES CHAMPION terminal screen visited via RAM shortcuts),
  L007 (reachability rejected: selector domain {0,2}), L008 (superseded).
- Produced (ready): L009 inline tables + case code (365 B), L010 fixed-bank math/VRAM
  library (298 B), L011 bank-31 header/residue (25 B), L012 unreferenced closed routines
  (1,798 B), L013 consumer-anchored tables (597 B), L014 data-interior generic data by a
  closure test (20,204 B), L015 unit-28 Data->Code correction (109 B).
- Gameplay: scenarios S09a-S09e (artificial season-end states, US terminal screen, orphaned
  JP sub-mode-19 scene) and S10 2P exhibition (natural) visited. Toolkit: TK-003 added.
- Next: disassembly pass validating L015, L009, L010, L012, L011, L013, L014.

## Coordinator: round 1 redirect -> round 2

- Round 1 redirect (attempt r1-a): COMPLETE. Ready leads L009-L015 cover all 23,287
  unknown bytes (handoff analysis/redirect/r1-a/handoff.md). Consumed L001/L002/L004/L006;
  L003 closed; L007 reachability rejected; L008 superseded. Gameplay: season end, Super
  Series ending, JP leftover scene (artificial), 2P exhibition visited.
- Round 2, phase: disassembly — STARTED (worker 2a). Input checkpoint pass-001; leads L009-L015.

## Round 2 disassembly handoff (worker 2a, 2026-11-18) - classification complete; editorial gates pending

ROM classification: 100.00% (393232 / 393232 bytes classified; 0 bytes unknown).

- Scope: one disassembly pass validating and applying ready redirect leads L009-L015
  (analysis/redirect/r1-a/handoff.md). No new gameplay (all prior scenarios S01-S10 reused;
  nothing unknown remained for play to resolve). commentary/ untouched (must rebase onto pass-002).
- Checkpoint: **pass-002** (maps/pass-002.json, generated/pass-002,
  analysis/pass-002-verification.json). Verification: ok=true, byte_identical=true; map sha256
  ad58fcd0504430d554a7182ae8fe5d4c6de88dd47ff0550e8541941f1efc24a9. map.load(state) ->
  annotate (2 removes + 1,104 annotations) -> map.validate ok -> map.save analysis/state.json ->
  map.export maps/current.json (identical to maps/pass-002.json). 4,154 regions, no overlaps.
  Backup of the pre-pass state is maps/pass-001.json (identical to the old state).
- Totals: Code 59,750 B (Observed 34,437 / Static 25,313), Data 177,886 (incl. 16-byte iNES
  header), Padding 24,524, Image (CHR) 131,072, Unknown 0. Execution coverage unchanged
  (17,278 observed instruction starts; separate metric).
- Method: analysis/round2-tools/props.py (proposal set from the r1-a ledger + L013/L015),
  check.py (independent validation against pass-001: proposals cover exactly the 23,287
  unknown bytes; each new Code region decodes exactly to its end, all branch/jsr/jmp targets are
  classified or co-annotated instruction starts, no fall-off, operands in RAM/PPU/APU/ROM;
  scan of ALL code (old+new) found no control flow into any proposed Data range; every inline
  table word is an instruction start), annotate.py (applied).
- Lead outcomes: L009, L010, L011, L012, L013, L014, L015 consumed (verified); none rejected.
  Details in analysis/leads.json (attempt records, pass round-2-disassembly).
  - L015: $ACC8 index bound re-derived (Y<=19); pass-001 Data 3024/3025 split; +109 B Code.
  - L014: all 43 A-style/A2 runs (1,761 B) inspected with context (music streams, text/stat
    records, metasprite records, tile maps) - accepted as generic data.
  - L011: checksums recomputed ($0EB1 PRG, $0D6A CHR). $FD41-$FD4D is the weakest item
    (decodes only as fall-in to Reset with discarded effects; DPCM-silence-like $AA bytes).
  - L012/L010 code: Static, reachability unproven (not claimed dead).

| bank | code | data | padding | unknown | newly classified in pass-002 |
|---|---|---|---|---|---|
| 0 | 1676 | 6516 | 0 | 0 | 878 |
| 1 | 0 | 7429 | 763 | 0 | 774 |
| 2 | 1676 | 6516 | 0 | 0 | 503 |
| 3 | 0 | 7221 | 971 | 0 | 628 |
| 4 | 1676 | 6516 | 0 | 0 | 647 |
| 5 | 0 | 7217 | 975 | 0 | 988 |
| 6 | 1676 | 6516 | 0 | 0 | 579 |
| 7 | 0 | 6957 | 1235 | 0 | 988 |
| 8 | 1167 | 7025 | 0 | 0 | 453 |
| 9 | 0 | 8064 | 128 | 0 | 602 |
| 10 | 155 | 8037 | 0 | 0 | 1004 |
| 11 | 841 | 4324 | 3027 | 0 | 309 |
| 12 | 155 | 8037 | 0 | 0 | 1268 |
| 13 | 830 | 2284 | 5078 | 0 | 528 |
| 14 | 155 | 8037 | 0 | 0 | 1010 |
| 15 | 356 | 4284 | 3552 | 0 | 787 |
| 16 | 2749 | 5443 | 0 | 0 | 742 |
| 17 | 4221 | 2875 | 1096 | 0 | 617 |
| 18 | 155 | 8037 | 0 | 0 | 1226 |
| 19 | 417 | 5972 | 1803 | 0 | 470 |
| 20 | 7448 | 744 | 0 | 0 | 786 |
| 21 | 1666 | 5575 | 951 | 0 | 727 |
| 22 | 4178 | 4014 | 0 | 0 | 1914 |
| 23 | 2882 | 2325 | 2985 | 0 | 373 |
| 24 | 494 | 7698 | 0 | 0 | 583 |
| 25 | 48 | 7640 | 504 | 0 | 551 |
| 26 | 7707 | 485 | 0 | 0 | 226 |
| 27 | 1657 | 6535 | 0 | 0 | 2219 |
| 28 | 6752 | 1440 | 0 | 0 | 176 |
| 29 | 3067 | 3739 | 1386 | 0 | 408 |
| 30 | 5382 | 2810 | 0 | 0 | 298 |
| 31 | 564 | 7558 | 70 | 0 | 25 |
(Bank = 8 KiB PRG bank; header 16 B Data and CHR 131,072 B Image not in the table.
Bank 28 newly-classified count excludes the 109 B L015 Data->Code correction.)

- Unresolved / weakest classifications (revisit if contradicted): L011 $FD41-$FD4D; L014 A2
  runs rest on a negative closure test plus context; specific semantics of most generic data.
- Reachability (separate from classification): 25,313 B Static code; L012 routines and five of
  six L010 routines have no known caller; units 2/6 streamer selection (L007) rejected.
- Deferred: commentary rebase onto pass-002 (new regions need labels/comments; ~1,600 generic
  branch labels from round 1), final editorial audit, natural password route (S09c).
- Toolkit: no new issues this pass; existing TK-001..TK-003 in analysis/toolkit-issues.md.
- Status: classification complete (0 unknown); overall ROM NOT declared complete - commentary
  readability and final editorial audit remain.

## Coordinator: round 2 disassembly -> commentary

- Round 2 disassembly: COMPLETE. Output checkpoint pass-002 (verified ok, byte_identical).
  Consumed L009-L015. Reported to user:
  ROM classification: 100.00% (393232 / 393232 bytes classified; 0 bytes unknown).
- Round 2, phase: commentary — STARTED (worker 2b): rebase commentary from pass-001 to pass-002.

## Round 2 commentary handoff (worker 2b, 2026-11-18) - readability complete; editorial audit pending

- Checkpoint pass-002 (sha256 ad58fcd0...24a9). Round-1 commentary preserved in
  analysis/round-1/commentary (verified copy); fresh baseline prepared from pass-002; source
  regenerated with carried knowledge plus tools/knowledge_r2.py.
- All newly classified code named/explained (L009 cases, L010 math/VRAM helpers, 42 L012
  routines with "No known caller; reachability unproven", L015 unit-28 code); all
  code-addressed data tables named; no raw ROM-address operands; generic data boundary notes.
- review.json: 4,154/4,154 entries and 1,803/1,803 units reviewed; editorial_audit pending.
- Generic branch labels 1,782 -> 1,222 (760 @Skip_, 294 @Loop_, 168 @Return_); ~37 numeric
  pointer immediates into unlabelled generic data remain (listed in the handoff).
- Fresh verify: ok, byte_identical true; reading copy refreshed. Final audit not run.
- Handoff: analysis/commentary-handoff-round2.md; commentary/progress.md, uncertainties.md.

## Coordinator: round 2 commentary -> redirect

- Round 2 commentary: COMPLETE (readability gate; handoff analysis/commentary-handoff-round2.md;
  round-1 commentary preserved in analysis/round-1/). Coordinator verify: ok, byte_identical.
  Remaining editorial items for final acceptance: ~1054 generic @Skip_/@Loop_ labels, ~37
  numeric immediate pointers into generic data.
- Round 2, phase: redirect — STARTED (worker 2c).

## Round 2 redirect (worker 2c, attempt r2-a, 2026-11-18) - no ready leads; no further disassembly cycle

- Checkpoint pass-002 (100% classified). Contradiction audit over the whole map
  (analysis/redirect/r2-a/consistency.json, inline-tables.txt, farcalls.txt, imm-ptrs.json):
  no flow target/read operand/observed fetch/bank context contradicts any Code/Data region;
  all 50 inline jump tables and 58/59 static far calls resolve to code starts (the 59th is the
  known U20_FarCallUnit18Entry4). DMC table confirms sample extents; $FD41-$FD4D (L011) lies
  outside every sample.
- New milestone S11: natural valid-password entry (000D6BD640) -> Super Series game 1;
  first fetches in 10 Static code regions, all inside Code.
- Leads: attempts appended to L005, L011, L014; no new or ready leads.
- Editorial-only items (DMC sample prose, password glyph letters) passed to final commentary audit.
- Handoff: analysis/redirect/r2-a/handoff.md (includes rejection review).

## Coordinator: round 2 redirect -> final acceptance

- Round 2 redirect (r2-a): no contradictions, no ready leads; rejection review in
  analysis/redirect/r2-a/handoff.md reviewed by coordinator (consistency audit over all code
  regions, inline tables, far calls, immediate pointers, DMC table; S11 natural password route).
  Sound: no new disassembly cycle. All bytes classified -> final editorial acceptance.
- Phase: final editorial acceptance — STARTED (worker 2d): generic labels, numeric pointers,
  DMC sample prose, password glyphs, then commentary.py audit.
- Phase: final editorial acceptance — COMPLETE (worker 2d, 2026-11-18). `commentary.py audit`
  ok: byte-identical, 4,154/4,154 regions and 1,803/1,803 units reviewed, editorial_audit
  reviewed; reading copy refreshed. All 1,054 @Skip_/@Loop_ labels renamed (202 hand names in
  commentary/tools/knowledge_r3.py, the rest from code-derived rules; 18 mechanical names remain).
  All 20 pointer pairs into generic data now labelled. DPCM sample prose fixed
  (DmcSample_DAC0_Tail). Password letters added. RAM reuse is shown with scoped names
  (Arg1A-1D, FieldPaintStep/Target, SavedBallRecord). Details: analysis/final-acceptance.md.
  Whole-ROM classification is unchanged (pass-002, 100%).

## Coordinator: TERMINAL HANDOFF (2026-11-18)

- Final editorial acceptance: COMPLETE. commentary.py audit ok, byte_identical true
  (commentary/audit.json); editorial_audit reviewed; analysis/final-acceptance.md.
- Final checkpoint pass-002 (maps/current.json sha256 ad58fcd0...24a9), map.validate ok,
  analysis/pass-002-verification.json byte_identical. input/game.nes sha256 unchanged.
- ROM classification: 100.00% (393232 / 393232 bytes classified; 0 bytes unknown).
- Workflow state: whole-ROM classification complete with final commentary audit and
  byte-identical verification. Residuals (documented, non-blocking): 18 mechanical branch
  labels, 168 @Return_ labels, rule-derived labels spot-checked only, generic data formats
  undecoded, reachability of 42 static-only routines + unit 2/6 copies unproven
  (commentary/uncertainties.md).
- Leads: L001-L015 all consumed/closed/rejected; no ready leads (r2-a rejection review).
- archive-options.json written (8 documents selected, 3 round-1 instruction copies omitted;
  every analysis/ and commentary/ Markdown file accounted for). Archive NOT executed.
