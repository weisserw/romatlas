# Commentary progress

Prepared verified baseline. All semantic review entries are pending.

Round-1 commentary (map pass-001) preserved unchanged in analysis/round-1/commentary/.
Round-2 commentary (map pass-002) preserved unchanged in analysis/round-2/commentary/.

## Method (carried from round 1)
- source/game.asm is rendered by tools/render.py from frozen before/game.asm plus
  annotations/*.txt. Edit annotations, then: `python3 commentary/tools/render.py`,
  `(cd commentary/tools && python3 review.py)`, toolkit verify and read. Never hand-edit
  source/game.asm; never edit before/.

## Round 2 (map pass-002) - COMPLETE for current source
- Fresh baseline prepared from pass-002 (byte-identical; 487 scaffold entries). Round-1
  tools and annotations copied and re-rendered against the new listing; all carried
  annotations land on instruction starts (render reports no warnings).
- New/changed code: annotations/nocaller.txt replaces unknown.txt. Headers for the 12
  no-caller ranges (0:$844C $9843 $98C6 $9A1A $9E30 $B091 $E0B7, 2:$A1EB, 3:$91B9 $9DBD
  $9E4A $A067) with mechanics and "no caller found, static only" notes; internal labels
  BypassedPal_*, MulSignedB3_NoAdd0-7/_Return, BulletHitB3_*. Removed data notes for
  3:$91B9 (DataB3_91B9) and 3:$9E4A (MulB3_Image), now code.
- Progression facts (redirect S07-S10) folded in: RAM $83 Landing -> MissionEndMode;
  CrashState $75-$7E is the docking sequence; FlightFrame labels Flight_Ground2/
  CrashCheck/Alive/FireCheck/Exploding/ExplodeEnd/ExplodeCrash/ExplodeTimer/Docking ->
  Flight_DockApproach/DockState/DockIdle/DockButton/DockSequence/DockCheckWindow/
  DockFailed/DockedWait/DockSparks; B+Up notes corrected (ground: climb-out; docking:
  skip to shop; battle: end, observed S10); DockingSetup no longer "probably".
- Stale "unknown bytes" continuation notes updated (Flight_DockedWait, Flight_FireButtons,
  SpaceFrame_FrameLeftStore, new StarReset_Position prose); header prose updated.
- review.json: 487 entries reviewed, 544 units; editorial_audit pending (final gate).
- Verify: byte_identical true; reading copy refreshed.

## Round 3 (map pass-003) - COMPLETE for current source
- Map changed pass-002 -> pass-003 (sha256 08865b21...76a3). Entire round-2 commentary
  copied to analysis/round-2/commentary/ (diff -r identical) before `prepare`; fresh
  baseline byte-identical, 482 scaffold entries.
- Carried by physical address: annotations + tools from round 2, re-rendered; no
  annotation lands on a non-instruction address and every data annotation matches a
  region start (checked). Diff vs round-2 source limited to the changed records below.
- Invalidated/replaced: data notes DataB1_D649, DataB3_D76E, DataB3_D7B8 removed (regions
  no longer exist). New: BlockSubstTables prose (8 x 16 B per mission, records 4-7 = 0-3,
  reads observed for all missions), BlockSubstTablesCopyB3 3:$D76F (unread copy),
  NoCaller_RtsAfterGroundUnderB3 3:$D76E (weak lone RTS, L009).
- Redirect r02-a facts: Flight_AutoLand / UpdateGroundObjects now "observed S11", carrier
  missions 4 and 8, $44 approach / $2C deck; MissionStart notes controller 2 cannot pick
  the start phase (S12); uncertainties items 1, 2, 6, 7 updated, item 9 added.
- Extra: ObjectHit_ObjectiveHit prose (objective hit remaps blocks $F7/$F8 to $6B).
- render.py: phase-1 one-line region comment is dropped when annotation prose exists
  (avoids duplicate prose on the two new named tables; no other output change).
- review.json: 482/482 entries reviewed, 544 units; editorial_audit pending (final gate).
- Verify: ok, byte_identical true (game.asm sha256 f9f5c2af...f91a, 24,274 lines);
  reading-after.asm refreshed.

## Final editorial acceptance (map pass-003)
- Report: commentary/editorial-audit.md. Fixes: 3:$D76E copied-RTS rationale (new render.py
  `regionnote` replaces the stale phase-1 region comment); block $6B wording -> observed S13
  mechanics; 104 address-named data labels renamed (<Table>_FillXXXX, FillBn_XXXX,
  UnrefMetaspriteSetB2, UnrefHeightMapsB3; DataB3_D90A kept); FarCall_B3 -> FarCallB3;
  uncertainties items 6/7/9 updated, 10 added.
- review.json: 482/482 entries, 554 units reviewed; editorial_audit reviewed.
- commentary.py audit: see commentary/audit.json.
