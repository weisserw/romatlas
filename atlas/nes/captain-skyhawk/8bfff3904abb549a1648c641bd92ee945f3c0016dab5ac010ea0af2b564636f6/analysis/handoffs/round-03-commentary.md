# Round 3 commentary handoff - COMPLETE (current-source readability, map pass-003)

## Scope
- Map changed pass-002 -> pass-003 (maps/current.json = analysis/state.json sha256 08865b21...76a3).
  Entire round-2 commentary preserved at analysis/round-2/commentary/ (verified identical with diff -r);
  analysis/round-1/ untouched. Fresh `commentary.py prepare` from pass-003: byte-identical, 482 entries.
- All current source covered: carried annotations re-rendered over the whole listing and reverified.

## Carried vs new
- Carried by physical address (commentary/annotations/*.txt + tools/render.py, review.py): all round-2
  names/comments/review. Checks: no annotation at a non-instruction address, every data annotation
  matches a pass-003 region start; source diff vs round 2 touches only the records below.
- Invalidated: DataB1_D649, DataB3_D76E, DataB3_D7B8 notes (regions removed).
- New/rewritten: 1:$D600 BlockSubstTables (8 missions x 16 B, records 4-7 = 0-3, 3/7 zero, reads
  observed for all 8 indices); 3:$D76F BlockSubstTablesCopyB3 (unread byte-identical copy);
  3:$D76E NoCaller_RtsAfterGroundUnderB3 (weak lone RTS, may be padding).
- Redirect r02-a folded in: Flight_AutoLand (0:$92DC) and UpdateGroundObjects (3:$D7EF) now state the
  S11 observation (carrier missions 4 and 8, $44 approach / $2C deck, landing -> MissionEndMode=1);
  MissionStart notes controller 2 cannot pick the start phase (Briefing_NextPhase overwrites Joy2Held,
  S12). uncertainties.md items 1 (landing closed), 2 (B+Down closed), 6 (no-consumer list trimmed),
  7 (adds 3:$D76E) updated; item 9 new. Header prose now cites pass-003, S07-S12, L006-L009.
- Small extra: ObjectHit_ObjectiveHit (3:$9F0D) explains the BlockSubst[7/8] := $6B write.
- render.py tweak (workspace-local): drops the phase-1 one-line region comment where annotation prose exists.

## Readability assessment
- Every code region has a named routine/continuation with prose (review.py: no code region without
  a unit); 0 Ln_XXXX labels; 0 raw non-immediate RAM/ROM operands. 105 address-named data labels remain
  (fill runs / generic regions with boundary notes). review.json 482/482 reviewed, 544 units;
  editorial_audit pending (final gate, not run).

## Verification
- `commentary.py verify`: ok, byte_identical true; source/game.asm sha256 f9f5c2af...f91a (24,274 lines).
- `commentary.py read`: commentary/reading-after.asm refreshed.

## Remaining gaps (meaning only; commentary/uncertainties.md)
- Reachability of the 13 no-caller ranges (incl. 3:$D76E). Data without consumer: 3:$D76F copy, 3:$D90A,
  2:$FE65, 3:$F1C3, 1:$FE16-$FE21. Undecoded internals (metasprites, spawn lists, maps, music); RLE
  overlaps (bank 1 $80C9, bank 0 $CA01/$CAF8). L002 RAM images remain Data (toolkit-blocked).
- Unverified: A/B star-speed bits in space; block $6B = destroyed-target picture (probable).

## Questions for redirect
- None blocking. Optional (low value): observe an objective hit to confirm the $F7/$F8 -> $6B block swap.
  Recommendation: proceed to final acceptance (editorial audit).
