# Round 5 — commentary handoff and final editorial audit

Status: **DONE**. Current-source readability is complete for checkpoint pass-005 and the strict editorial
audit passed. Whole-ROM coverage is 262,160/262,160 bytes, but **3 of those bytes are classified only by a
user-directed manual fallback**, not by evidence (see "Manual fallback" below).

## Scope and checkpoint
- Checkpoint pass-005 (maps/pass-005.json = maps/current.json = analysis/state.json, sha256 e401cd2f...).
- The only map change from pass-004: bank-1 bytes 0x02DC6 ($ADB6), 0x032FA ($B2EA) and 0x03353 ($B343) went from
  Unknown to regions 3315-3317 (Data, Manual, confidence 0.0).
- Round-4 commentary, including its pass-004 final audit, was preserved first in analysis/round-4/commentary/
  (diff -r identical, source/game.asm sha256 54734f86...), with the round-4 commentary_build.py,
  commentary_review.py, r04c tools (r04c-tools/), the round-4 handoff and a README.md.
- `commentary.py prepare` re-created commentary/ from pass-005 (byte_identical baseline, 3,272 scaffold entries).
  notes/annotations.txt, progress.md, uncertainties.md and editorial-audit.md were restored from round 4.

## Carry results
- analysis/tools/r05c/carry.py (adapted from r04c, compares against analysis/round-4/commentary/before/game.asm);
  output in analysis/evidence/round-05/commentary-carry-check.txt.
- All fn 522, lbl 734, cmt 23, op 65, xb 43 and dat 205 records still hit the same instruction or region;
  6 of 9 rgn records are valid. Invalidated: only the 3 former unknown-range notes (region kind changed).
- review.json: 3,272 entries, all reviewed. 3,269 were carried from round 4 with the same range, kind and summary;
  3 (regions 3315-3317) were re-reviewed. Units: 1,081, all reviewed and identical to round 4.
- The rebuilt source differs from the audited pass-004 source only in the title line and the 3 notes.

## Source and document changes
- The 3 notes now say: a single $60 byte after the tail `jmp` of FightFish_Update_fish_away / FightFish_Passive /
  FightFish_DirectionTerms, classified as generic data of unknown meaning only by user-directed manual fallback
  (Manual, confidence 0.0), with no evidence of a data role and no known reader or format; L-023 (code) was
  rejected in round 4, nothing references it in any searched encoding, the r04-a runtime watch saw no access,
  so code versus data remains open. No label was added. They are not called a table, padding, dead code or unused.
- Title line: round 5 / checkpoint pass-005 (builder analysis/tools/commentary_build.py). Review tool defaults
  now refer to round 5 and compare against the round-4 archive.
- uncertainties.md: round-5/final section added; the round-4 and pass-004-audit "3 unknown bytes" statements are
  kept as history and marked superseded. progress.md: round-5 section. editorial-audit.md: round-5 section
  appended (pass-004 content preserved).

## Counts
- Unknown bytes: 0; unknown-range notes: 0; Manual-data notes: 3.
- Readability (analysis/evidence/round-05/commentary-labstats.txt; game.asm 35,010 lines): address-only names 0,
  numbered generic locals 0, global `_N` suffixes 0, raw RAM operand lines 0; 1,257 globals, 686 locals;
  707 routine headers, 343 header-less named stubs/continuations.

## Verification
- `commentary.py verify`: ok true, byte_identical true; game.asm sha256
  e06730f4c0a98d6548c823ddb12cf09b86070baa21fb1c68fea90d6e061ab5de (commentary/verification.json).
- `commentary.py read`: commentary/reading-after.asm refreshed.
- `commentary.py audit`: ok true, byte_identical true, editorial_audit true, reviewed_regions 3272,
  reviewed_units 1081 (commentary/audit.json); reading-after.asm sha256 92e41c33...
- input/game.nes unchanged (sha256 dfaed2d0...); maps/ and analysis/state.json not touched.

## Manual fallback (reported separately from proven classification)
- Evidence-backed (Observed/Static) classification: 262,157 bytes.
- Manual user-directed fallback: 3 bytes (0x02DC6, 0x032FA, 0x03353). Their meaning is unknown and their
  code-versus-data role was not established by evidence.

## Unresolved
- Unchanged from round 4 (commentary/uncertainties.md): FishState $A0; FishAheadHandlers facing 8;
  FatigueDrainFlag and BitArrayPtr writers; OffsetPair16/18 operand roles; species per HookedFishType;
  fight-model quantities; sound semantics; HarbourReturnFlag meaning.
- 482 data regions keep only generic phase-1 notes; 343 named stubs/continuations have no header.

## Not done / out of scope
- analysis/progress.md not edited (coordinator). No archive, publish or delete. Toolkit not modified;
  no toolkit issues found (analysis/toolkit-issues.md unchanged).
- Scratch: the temporary moved-aside copy of the round-4 commentary (identical to the archive) was removed.
