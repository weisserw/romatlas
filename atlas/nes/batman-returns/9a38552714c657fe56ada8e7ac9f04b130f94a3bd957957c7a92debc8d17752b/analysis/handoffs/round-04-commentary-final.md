# Round 4 commentary + final editorial audit (pass-004) - AUDIT PASSED

Baseline: checkpoint pass-004 (maps/pass-004.json == maps/current.json == commentary/baseline.json).
Readability, editorial acceptance and ROM classification are reported separately below.

## Scope
- Whole current source commentary/source/game.asm (single translation unit, 45.9k lines in the reading copy),
  review.json 1909 scaffold entries and 2117 units.

## Baseline change handling
- pass-003 -> pass-004 added only regions 1907-1909 (1-byte Data, Manual, conf 0.3, no label) at physical 0x14CCA
  (bank 10 $8CBA), 0x17BC6 (bank 11 $BBB6), 0x1FD5A (bank 15 $FD4A). The scaffold keeps 1909 entries; the three
  unknown-* entries became region-1907..1909; every other entry is identical.
- Round-3 commentary preserved first: analysis/round-3/commentary/ and analysis/round-3/tools-commentary/
  (diff -r identical; analysis/round-3/README.txt). analysis/round-1/ and round-2/ untouched.
- Fresh `commentary.py prepare` (byte_identical) -> new before/, baseline.json, phase1-state.json, review.json.
- Carried by physical identity (ann keys = bank:CPU): ann/*.txt, uncertainties.md, progress.md. jtables.py re-run
  (193 tables, identical to round 3). First re-render differed from round 3 only at the preamble and the three
  changed bytes; all other names, comments and review credit reused. Invalidated/re-reviewed: regions 1907-1909
  and their neighbours (IntroStep12_Text / IntroStep13_Objects, BossW6_MoveRun / BossW6_Fight, SetScrollAndCtrl /
  DisableNmi).

## Edits
- Fallback bytes: Data-header notes (new `dnote` directive in build_ann.py/render.py; the generator's duplicate
  evidence line is dropped) state: emitted as a data byte by user-directed manual fallback, meaning unknown, no
  observed/proven data role, value $60 = RTS opcode, code-vs-data ambiguous (L017), no reachability claim,
  links analysis/handoffs/round-04-fallback.md and round-03-redirect.md. review.json summaries match (review.py
  FALLBACK). Preamble updated to pass-004 (no unclassified bytes remain).
- Renames (old name contradicted the code): BossW2_Hit -> BossW2_StartAttack, BossW6_Hit -> BossW6_StartAttack,
  BossW2_Defeat -> BossW2_IntroEnd, BossW4_Hit -> BossW4_IntroLeap (tentative reading).
- 10 stale round-1 comments stacked on contradicting later descriptions removed (ann/90-notes.txt, 92-flow.txt).
- Stale names/addresses in prose fixed: BankPushAndMapY, DamageDoubler, AttractStoryUpdate (Intro_RunStory),
  Ending_Run (EndingUpdate caller), StageRam650 (+ corrected RAM note), BgAnimLists in bank 10, UpdateAllObjects
  loop description, PlayerHurtStart/PlayerHurtUpdate, PlayerEntryAnimDispatch, credits steps $0E/$0F, StageState.
- Tables named from existing documentation: AttractStoryTable, IntroFallerState/X/Y, BgAnimLists,
  EndingCreditScriptIds. render.py: high-byte reads of named .word tables render as Table+1 (20 stale
  Data_Bnn equates removed; 297 Data_Bnn equates remain).
- RAM usage notes of generic RAM symbols regenerated from the current source (tools-commentary/ramusers.py).
- Comments added for every routine unit that had none (~40); ~12 thin descriptions of large routines expanded
  (BossFight, BossW2_Chase, BossW6_Approach, BossW4_AimRun, VehObjRoadSpeed, VehicleStageInit2,
  VehicleBossUpdate, PpuScriptQueueCopy, EnemyC_Jump, CutsceneTimerReset, SndVibReadByte, ObjMotionScriptStart).
- Files: commentary/ann/{00-ram,04,14b,15,00-sound,90-notes,92-flow,p2-04,p2-05,p2-11,p2-obj,p2-ramnotes,p2-sound,
  r2-10,r2-11,r2-14,r2-15,r2-unknown}.txt, new commentary/ann/r4-final.txt; analysis/tools-commentary/
  {build_ann,render,review}.py, new ramusers.py; commentary/uncertainties.md ("Round 4" section, L017 item and
  stage 3-3 note updated in place), commentary/progress.md (Round 4 section).
- Regeneration: `sh analysis/tools-commentary/rebuild.sh` (stable, re-run gives identical source), then
  `EDITORIAL_NOTES=<notes file> python3 analysis/tools-commentary/review.py` (without the variable review.py resets
  editorial_audit to pending).

## Audit result (`commentary.py audit`, exit 0)
- ok: true; byte_identical: true; reviewed_regions: 1909; reviewed_units: 2117; editorial_audit: true.
- editorial_audit in review.json: status reviewed, findings recorded in its notes (checks performed, defects fixed,
  accepted residue).

## Verification
- `commentary.py verify` (fresh, after audit): ok true, byte_identical true; commentary/verification.json.
- commentary/source/game.asm sha256 712a9ad5f3c6dd06532de821c59f9a7b699886f75d77220c0729b6f8779ee8b9.
- `commentary.py read`: commentary/reading-after.asm refreshed. input/game.nes sha256 9a385527... unchanged.
- Standard checks: 0 Sub_Bnn / region_ names, 0 numeric zero-page operands, raw switchable-bank operands 2
  (documented: $8000 bank-ID read, $8159 logo checksum).

## ROM classification (separate from commentary)
- Proven/evidence-backed classification: 393229 / 393232 bytes (as at pass-003).
- Manual user-directed fallback: 3 bytes (regions 1907-1909), generic Data with UNKNOWN meaning by explicit batch
  authorization; NOT an observed or statically proven data role; code-vs-data remains ambiguous.
- Map total: 393232 / 393232 bytes classified (100.00%), of which 3 bytes are the fallback above.

## Remaining semantic uncertainties (documented, not blocking the editorial gate)
- Per-state game meaning of ~640 structurally named dispatch entries; enemy/boss character identities.
- 297 Data_Bnn equates and generic object/boss fields (ObjVar418..4F4, BossVar508-50B) with mechanical notes.
- Wave-script terminators; whole-bank generic data (banks 1-3, 8, most of 9).
- BossW2 intro entries Start/Throw/TakeHit/Flash (scripted or live); BossW4_IntroLeap reading; BossTimerAdvance
  unindexed ObjTimer; the three fallback bytes (meaning unknown).
- No runtime run in this phase.
