# Handoff: round 6, phase 2 (commentary)

**Status:** the current-source readability gate is met for all classified source on the
round-6 map. A fresh byte-identical verification passed after the last edit and the reading
copy is refreshed. The final editorial audit is NOT accepted: review.json `editorial_audit`
stays pending for final acceptance. Whole-ROM classification (in the map) is 393,232 of
393,232 bytes, 0 unknown; this pass changed no classification.

## Checkpoint
- Map: maps/round6-disassembly.json (= maps/current.json = analysis/state.json, sha256
  a5b53604...). input/game.nes unchanged (sha256 bd987bf8...). Toolkit not modified. No stale
  edit process was running.
- Round-5 commentary preserved in full in analysis/round-5/commentary/ (`diff -r` identical
  before the old directory was removed). Earlier round copies untouched.
- Fresh baseline from `commentary.py prepare` (2,248 scaffold entries); before/ frozen.
  Tools, specs, progress.md and uncertainties.md carried from round 5.
- Source: commentary/source/game.asm (sha256 06193f47...), reproducible from the round-5
  source by commentary/tools/merge_r6.py, then commentary/tools/edits_r6.py.
- leads.json backed up to analysis/round6/leads.before-commentary.json.

## Verification
- `commentary.py verify`: ok, byte_identical true (final run after the last edit).
- `commentary.py read`: commentary/reading-after.asm refreshed.
- `commentary.py audit` (run for information only): stops only at "final editorial audit is
  pending", as intended. Not an accepted audit.

## Scope and readability
- Every block with an unchanged physical range is carried verbatim. The exceptions are the
  label-only group-F blocks, which got new comments.
- Changed ranges, written by hand:
  - TeamRosterPtrs (`.word` list) and TeamRoster00..11. The 16-byte roster record is
    documented once at TeamRoster00, with new RREC_* constants used at 31 fixed-offset
    ReadTeamRosterByte calls. Fields: flags, initial, run speed, lift, hitting, batting form,
    name characters $06-$09/$0F, home runs and average (both tentative), and birth date
    $0C-$0E.
  - DefaultLineupPtrs/DefaultLineup00..11: the 21-byte lineup record (9 player/position pairs
    plus 3 rotation starters) is documented at DefaultLineup00.
  - Close-up tables: the three-level scheme (side -> form -> 32-word frame table) is
    documented at CloseupMetaspriteSidePtrs. The side and form tables are `.word` lists. The
    generic block from $CA4A has a boundary note.
  - The six Layout_* tables, PitcherFormChrBank, and 15 group-F tables, each with its consumer,
    index domain and record size.
  - All 16 generic remainders have role/uncertainty notes. Round-5 source labels inside them
    were re-defined at the same offsets.
- Old -> new names were remapped as the disassembly handoff listed: D_B02_E811/E812,
  D_B02_ED89 -> TeamRoster03+176, D_B14_C0FA/C5A2 (+1), Layout_*Hi -> +1, and 15
  second-byte aliases -> Label+1. All 57 new map labels exist in the source.
- Prose fixes:
  - BuildPitcherRecord: its old "pitcher*2+6" wording was wrong. It copies roster bytes
    $06-$0B and $0E.
  - Also updated: PlayerRecordPtr(A), LoadPlayerBiorhythm, FillPitcherBiorhythm,
    DrawScoreAndCount, DrawPitchSpeed, DrawBatterSprites, ResetPitchState and DrawBatterInfo.
- review.json: 2,248/2,248 entries reviewed, 1,235 units.

## Leads
- None produced or consumed. A pass entry was added to analysis/leads.json. L-025 commentary
  is done.

## Unresolved (commentary/uncertainties.md, round-6 section)
- Weaker evidence is stated in the source:
  - D_B07_CD0D: entries 12-23 come only from the pointer stride (the consumer indexes 0-11).
  - D_B06_CC82: the index domain was not traced, so its extent rests on the parallel table
    D_B06_CCA2.
- These interpretations are tentative: roster bytes $02-$05, $0A and $0B; bit5 of byte 0
  (probably the throwing hand); lineup position numbering.
- Not established: which AtBatFlags bit7 state each close-up side table serves; record
  boundaries in the generic block from $CA4A; the word table at $D0D9.
- The round-5 "$FA14-$FAE8 unidentified" item is closed: those bytes are the TeamRoster11 tail.

## Deferred
- Final editorial acceptance (`commentary.py audit` with editorial_audit reviewed).
- Toolkit: no new issues (TK-001..TK-005 unchanged).
