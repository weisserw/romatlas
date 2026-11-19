# Commentary progress

## Round 2 (baseline pass-002) - 2026-11-19
- Map changed since the round-1 baseline (pass-001 -> pass-002, ~780 new regions). The whole previous
  commentary directory was copied to analysis/round-1/commentary (plus analysis/round-1/tools-commentary)
  and verified identical before `commentary.py prepare` created this fresh baseline (before/, baseline.json,
  phase1-state.json frozen at pass-002; prepare verify byte_identical).
- Carry-forward: annotations are keyed by (bank, CPU address) = physical identity, so commentary/ann/*.txt
  from round 1 was copied and re-rendered against the new before/game.asm. All round-1 label/comment/
  operand keys landed on instruction starts except one comment ($8368 -> $836E, StageLoadGraphics).
  Stale statements about unknown/unclassified ranges, descent roots and wrong roles were rewritten.
- Tooling changes (analysis/tools-commentary): jtables.py derives the 193 code-pointer tables from map
  evidence (replaces traces/round1/static-tables.json; per-entry pair overrides for ObjHandlerTable and
  StageEventDispatch); gen_auto.py writes ann/00-auto.txt (structural names/comments for table entries,
  overridden by manual files); rebuild.sh runs build/gen/render; newsum.py/jtinfo.py are review helpers;
  typeprobe.py = bounded ObjType census per stage (typeprobe-out.json, artificial starts).
- New annotation files: r2-04, r2-05, r2-06, r2-07, r2-10, r2-11, r2-12, r2-14, r2-15, r2-misc, r2-unknown.
- Scope covered: every routine entry in the 780 new regions is named (0 Sub_Bnn names, 0 numeric RAM
  operands, all switchable-bank operands resolved except 4 documented ones); owners of all 193 dispatch
  tables are explained; the 9 L017 unknown bytes carry notes.
- Renames and interpretation changes: see uncertainties.md "Round 2".
- Verification: commentary.py verify ok, byte_identical (see verification.json); reading copy refreshed.
- Deferred / not done: per-state game meaning of ~640 structurally named dispatch entries (role at the
  owner, generated mechanics line); 322 Data_Bnn equates keep mechanical names with consumer notes;
  local branch labels in new code are systematic (_Skip/_Loop/_Return). Editorial audit pending (final
  acceptance only).

## Round 3 (baseline pass-003) - 2026-11-19
- Map changed (pass-002 -> pass-003: 6 bytes unknown -> Code, regions 1903-1906; scaffold otherwise
  identical). Round-2 commentary preserved first: analysis/round-2/commentary + analysis/round-2/tools-commentary
  (diff -r identical), then `commentary.py prepare` (byte-identical) built this baseline.
- Carry-forward: ann/*.txt, uncertainties.md, progress.md copied; jtables.py re-run (193 tables, identical);
  rebuild produced source that differs from round 2 only at the 4 changed ranges. Invalidated/re-reviewed:
  the 4 new Code records and their host routines (PlayerDeathOrRecover, StageEvt_Stage3_1Exit_Fade,
  StageEvt_FinalBossCleared, Ending_Run); the 3 still-unknown notes rewritten. All other review credit reused.
- L019 applied: ObjT33 operands symbolic (ObjT32_Handler_St00_St01_Skip / ObjT03_Spawn, pair 6/7), stage $11
  (5-1) wave-script origin of types $32/$33, StageEventIndex 3 notes (StageEvt_Stage3_1A, StageLoadGraphics,
  StageEventDispatch, StageEventByIndex), boss-loop census at ObjHandlerTable/UpdateAllObjects, wave-record note.
- review.py: explicit summaries for the 4 dead-tail records. Verify ok/byte-identical; reading copy refreshed.


## Round 4 (baseline pass-004) + final editorial audit - 2026-11-19
- Map changed (pass-003 -> pass-004): three 1-byte regions added (1907-1909, Data, Manual, conf 0.3, no label) at
  physical 0x14CCA (bank 10 $8CBA), 0x17BC6 (bank 11 $BBB6), 0x1FD5A (bank 15 $FD4A). These are a user-directed
  fallback (explicit batch authorization), not an observed or proven data role. Scaffold: 1909 entries; the three
  unknown-* entries became region-1907..1909; all other entries identical.
- Round-3 commentary preserved first: analysis/round-3/commentary + analysis/round-3/tools-commentary (diff -r
  identical; README.txt). Then `commentary.py prepare` (byte_identical) built this baseline.
- Carry-forward: ann/*.txt, uncertainties.md, progress.md copied; jtables.py re-run (193 tables, identical to round 3).
  First re-render differed from round 3 only at the preamble and the three changed bytes; all other review credit
  reused. render.py/build_ann.py gained a `dnote` directive so the Data header of a classified region can carry
  its own note (replacing the duplicate generator evidence line).
- Fallback bytes: source notes rewritten (data byte by user-directed manual fallback, meaning unknown, $60 = RTS
  opcode, code-vs-data ambiguous, no reachability claim, link to analysis/handoffs/round-04-fallback.md);
  review.json summaries say the same. Preamble no longer mentions "unclassified bytes" (none remain).
- Final editorial pass (concrete defects fixed): 4 misleading routine names (BossW2_Hit, BossW6_Hit,
  BossW2_Defeat, BossW4_Hit) renamed; 10 stale stacked round-1 comments removed; stale names/addresses in prose
  fixed (BankPushAndMapY, DamageDoubler, AttractStoryUpdate, Ending_Run, StageRam650, BgAnimLists bank,
  UpdateAllObjects loop description, PlayerHurtStart/Update, PlayerEntryAnimDispatch, credits steps);
  5 tables named; 20 stale high-byte equates replaced by Table+1; RAM usage notes regenerated
  (tools-commentary/ramusers.py); comments added to every routine unit that had none (about 40) and about a dozen thin
  routine descriptions expanded (bosses, vehicle stage, PPU string copy, sound vibrato reader, scene timers).
- Inventory: 1909/1909 entries and 2117/2117 units reviewed with summaries and source lines; editorial_audit
  recorded as reviewed; `commentary.py audit` run (result in analysis/handoffs/round-04-commentary-final.md).
- Remaining semantic uncertainty: see uncertainties.md "Round 4".
