# Round 2 disassembly handoff

## Scope
One disassembly pass consuming the round-1 redirect's ready leads L8, L2, L13 and L14. Input
checkpoint pass-002 (100.00%, byte-identical). No new gameplay was driven: all milestones,
including blackjack, the King match and the ending, were already visited (scenarios S1-S13).
The remaining Static-only code (5624 bytes) affects provenance only, and traced play is slow
(TK-004).

## Checkpoint / verification
- pass-003: maps/pass-003.json (sha256 9e2a7bca...22f2), generated/pass-003,
  analysis/pass-003-verification.json ok=true, byte_identical=true.
- map.validate ok; analysis/state.json == maps/current.json == maps/pass-003.json.
- ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).
  Code 31825 (Observed 26201 / Static 5624), Data 99263. Per-bank table in analysis/progress.md.
- Map build: python3 tools/mkmap.py -> scratch/newmap.json -> map.load/validate/save/export.
  Pre-round-2 copies: scratch/mkmap.pre-round2.py, scratch/state.pre-round2.json,
  scratch/leads.pre-round2.json, traces/coverage.pre-round2.json.

## Findings / lead outcomes
- L8 verified: $CFF6 (physical 118790), 1 byte, Data -> Code/Static. Region $CFE6-$CFF6 is now
  17 bytes. This is the only instruction-start change. Reachability is unresolved.
- L2 verified: redirect coverage merged into traces/coverage.json (12688 opcode starts). The
  re-check found 0 conflicts. 9419 bytes Static -> Observed. mkmap splits runs per instruction,
  so this is more than the 5219-byte full-region estimate. Some of the merged sessions were
  entered through recorded RAM patches. That counts as observed execution, but it does not show
  natural reachability.
- L13 verified: bank 5 CheatPwRichman/CheatPwSeeKing/CheatPwBankrupt/CheatPwDebugMonitor (4 x 30
  at $93DF). Bank 6 PresetPasswordPtrs ($87DA, 8 bytes) and PresetPwRichman/SeeKing/Bankrupt/
  DebugMonitor (4 x 30 at $87E2). The zero-run region inside the first cheat string was absorbed.
- L14 verified. Bank 3: BjDealerNamePtrTable $913A, BjDealerNameStrings $914A, unlabelled UI
  text $9177, BjDealerRecordTable $91D4, BjMessagePtrTable $9264, BjMessageTexts $94C4-$9B87.
  Bank 4: PkOpponentRecordTable $9F06, PkOpponentTriples $9F57, unlabelled text $9F72,
  PkHandNameRecords $A038, PkOpponentMsgBytes $A064, PkMessagePtrTable $A06D,
  PkHandNameStrings $A373-$A407, unlabelled text $A408, PkMessageTexts $A597-$BABF.
  Correction to redirect notes D: the hand-name strings end at $A408, not $A405.
- Leads produced: none. No new toolkit issues were found (TK-001..TK-004 are unchanged; see
  analysis/toolkit-issues.md). Observation: data regions reject execution views, so data labels
  are carried without views.

## Region changes the commentary must absorb (frozen baseline commentary/before is pass-002)
1. Re-prepare the commentary from pass-003. The region count went from 1493 to 1582.
2. Code regions are re-split at Observed/Static instruction-run boundaries in banks 3-7. The
   kinds and instruction starts are unchanged, apart from $CFF6.
3. $CFF6 is now code (PHA) inside the $CFE6 fragment. Drop the data note for it. A label at
   $CFF6 is now valid.
4. There are 21 new map data labels (names above), with no collisions with the current .ann
   D-labels. Either adopt them or rename them consistently. The data blocks that were split:
   bank 5 $93DF-$9456 (old 3 regions), bank 6 $87DA-$8859 (old 3), bank 3 $913A-$9B87 (old 1),
   bank 4 $9F06-$BABF (old 1).
5. Commentary fixes still pending: L9 (ROYAL STRAIGHT), L10 (blackjack semantics), L11 (debug
   buttons A/B), L12 (GAME OVER), L13 (the "$87F0 four 28-char strings" wording is wrong; the
   strings are at $87E2, 30 bytes each, and 28 are compared).

## Unresolved / deferred
- 5624 Static-only code bytes, mostly bank 4 AI/King branches and bank 3/5/6/7 leftovers.
  Their classification rests on static flow; only provenance is open.
- L4 (far-call trampoline) and L7 (unreferenced fragments): reachability is open. L5/L6:
  graphics and fixed-bank data semantics, labels only.
- Field meanings of PkOpponentTriples, PkOpponentMsgBytes and the record fields. Consumers of
  the three unlabelled text spans.
- FINAL HAND timing and the natural "all opponents beaten -> King" trigger (low value).
