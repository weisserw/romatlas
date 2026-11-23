# Commentary progress

Prepared verified baseline. All semantic review entries are pending.

Round-1 commentary (map pass-001) preserved unchanged in analysis/round-1/commentary/.

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
