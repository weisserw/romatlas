# Final editorial audit — checkpoint pass-004

Date: 2026-11-20. Scope: a targeted final pass over commentary/source/game.asm, which is built from
commentary/notes/annotations.txt by analysis/tools/commentary_build.py. Review credit from rounds 1-4 was
reused; this was not a whole-source reread. The pre-audit copies of the source, annotations, builder, review.json,
progress.md and uncertainties.md are in scratch/final-acceptance/pre/.

## Checks performed
- Address-only or numbered symbols. All equates and labels were searched for hex-suffix names.
- Symbol consistency across banks. Equates sharing a value were checked; only constant/RAM overlaps and the
  documented $D0 aliases (EntryRow / TitlePictureStep / IntroObjSpeed) were found.
- Stale round and pass references. All of them are historical attributions. The title line says round 4 / pass-004.
- Overclaims. The search covered "unused", "dead", "unreachable", "never", "no writer", "not written anywhere",
  and "observed" wording inside Static code regions.
- Notes on unknown ranges. All 3 were checked against L-023 and the round-04 rejection.
- The commentary/uncertainties.md register.

## Defects fixed (all edits went through annotations.txt or the builder, then a rebuild)
1. Address-only RAM names. The round-4 handoff counted 0 such names, but its count missed these equates:
   - Var07AF -> EngineLevel (boat engine level, Boat_EngineSound);
   - Var0500 -> RamPage05 (base of the page cleared at reset; no other use);
   - Var05F6 -> HarbourReturnFlag. This one is tentative: Fight_ReturnToSea goes to the harbour when it is
     nonzero, and bank-12 $98C3/$98CA bytes read as ScrCmd_Poke $00/$80;
   - GameFlags04A0 -> TourWrapFlags (bit 0 is set when TournamentRound wraps after round 4, and the round-4
     target weight is used afterwards; both paths are static).

   The derived local labels follow the new names (for example @EngineLevel_nonzero).
2. ScriptArgC6/C8/CA had empty comments. Their roles are now stated. The names are kept because they are
   consistent with Script_ReadWordC6/C8/CA and Script_CallC6.
3. ScrCmd_End said "Command $00/unused entries". It now names the exact table slots that lead to it ($16/$17) and
   notes that a $00 byte stops the interpreter before dispatch. The $E5FD end byte is no longer described with
   the handler's name.
4. The unknown note at $ADB6 suggested "a dead epilogue left over from a jsr->jmp edit". The L-023 twin test did
   not establish that edit history, and the note now says so.
5. FatigueDrainFlag and BitArrayPtr were described as "no writer in the ROM" / "not written anywhere". They now say
   "no writer found". Absolute operand bytes were rechecked for this audit: $05A5 has only the single read, and
   the $84 store patterns occur only inside data.
6. The L-016 notes for whole-bank data said "bank N is never mapped for execution". They now read "no bank-select
   path maps bank N for execution (L-016 static bank-select audit)". This was a builder text change covering
   banks 3, 5 and 8-11.
7. The bank-12 data at $98B5-$98D0 had only a generic note. The new note says it is probably script bytecode that
   resumes after ScriptInline_TimeUpTest, and it shows the Poke $05F6 reading. The phase-1 Data kind is unchanged.
8. uncertainties.md: the round-3 note about "5 unknown bytes" is marked as superseded (the NOPs became code in
   round 4), "dead" is reworded, the no-writer claims are softened, and a final-audit section was added.

review.json was refreshed with analysis/tools/commentary_review.py. Its scaffold, statuses and pass fields are
unchanged; only the summaries and source line numbers changed. All 3,272 entries and 1,081 units are reviewed.

## Verification
- `commentary.py verify`: ok, byte_identical true.
- `commentary.py audit`: see commentary/audit.json (ok, byte_identical, editorial_audit true). This also
  refreshed commentary/reading-after.asm.

## Residual limitations
- **Whole-ROM classification is NOT complete.** 262157/262160 bytes are classified. The 3 unknown bytes are lone
  RTS opcodes in bank 1, at 0x02DC6 ($ADB6), 0x032FA ($B2EA) and 0x03353 ($B343). Each has a source note, and
  L-023 was rejected (analysis/handoffs/round-04-redirect.md). This audit does not reclassify them.
- 482 data regions carry only the generic phase-1 rule note ("consumer/format not identified").
- 343 named stubs and continuations have no routine header.
- The open semantic items in uncertainties.md are unchanged. They include FishState $A0, the FishAheadHandlers
  facing-8 entry, the species for each HookedFishType, the fight-model quantities, sound semantics and the
  HarbourReturnFlag meaning.
- The audit tool checks inventory, review status and byte identity only. Prose quality rests on this manual review.

# Final editorial audit — checkpoint pass-005 (round 5)

Date: 2026-11-20. The pass-004 audit above is preserved unchanged as history; where it says the 3 bank-1 bytes
are Unknown, that is superseded for classification coverage by pass-005 (see below).

## Scope
- The map changed from pass-004 to pass-005 only for the 3 bank-1 bytes 0x02DC6 ($ADB6), 0x032FA ($B2EA) and
  0x03353 ($B343). They became regions 3315-3317, Data, provenance Manual, confidence 0.0, by a user-directed
  manual fallback (analysis/handoffs/final-fallback.md). This is not an observed or statically proven data role.
- The audited pass-004 commentary was archived in analysis/round-4/ and commentary/ re-prepared from pass-005.
  Annotations were carried by physical range (analysis/evidence/round-05/commentary-carry-check.txt); only the
  3 former unknown-range notes were invalidated. The rebuilt source differs from the audited pass-004 source only
  in the title line and those 3 notes, so pass-004 audit credit was carried for everything else.

## Checks performed
- The 3 new notes: they say generic data of unknown meaning, classified only by the Manual fallback; they keep
  the prior ambiguity (L-023 rejected, no reference in any searched encoding, 0 accesses in the r04-a runtime
  watch) and say code versus data remains open. No label was added. They do not call the bytes a table,
  padding, dead code or unused, and do not claim evidence for a data role.
- No `; Unknown;` region headers remain in the source; the builder reports 0 unknown bytes.
- Stale references: the source title line said round 4 / pass-004 and now says round 5 / pass-005. The
  "3 unknown bytes remain" statements in uncertainties.md (round 4 and final-audit sections) and progress.md
  are kept as history and marked superseded by pass-005; a round-5 section was added to uncertainties.md.
- Readability counts (analysis/evidence/round-05/commentary-labstats.txt) are unchanged from round 4:
  0 address-only names, 0 numbered generic locals, 0 global `_N` suffixes, 0 raw RAM operand lines.
- review.json: 3,272 scaffold entries (3,269 carried from round 4 with the same range, kind and summary; 3
  re-reviewed) and 1,081 units, identical to round 4; all reviewed.

## Defects fixed
1. The 3 notes were rewritten from "Unknown" wording to the Manual-fallback wording above.
2. Source title line round/pass corrected.
3. Superseded markers added to the stale unknown-byte statements in uncertainties.md and progress.md.

## Verification
- `commentary.py verify` and `commentary.py audit`: results in commentary/verification.json and
  commentary/audit.json, and in analysis/handoffs/round-05-commentary.md.

## Residual limitations
- 3 bytes (0x02DC6, 0x032FA, 0x03353) are classified only by user-directed Manual fallback. Their meaning is
  unknown and their code-versus-data role was not established by evidence. Evidence-backed classification
  covers 262,157 of 262,160 bytes.
- 482 data regions keep only generic phase-1 notes; 343 named stubs/continuations have no header.
- Open semantic items in uncertainties.md are unchanged.
- The audit tool checks inventory, review status and byte identity only; prose quality rests on this manual review.

