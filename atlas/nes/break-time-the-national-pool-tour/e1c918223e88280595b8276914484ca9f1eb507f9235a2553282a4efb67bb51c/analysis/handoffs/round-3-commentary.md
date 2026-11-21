# Round 3 commentary handoff

Status: **COMPLETE** for current-source readability on checkpoint pass-003.
Classification is unchanged at 99.99%: 163,845 of 163,856 bytes, with 11 bytes
unknown (lead L-017). Classification is reported separately from readability. The
final editorial audit and the archive were not run.

## Scope
- The map changed between pass-002 and pass-003.
  - The full round-2 commentary was moved, untouched, to analysis/round-2/commentary/.
  - The round-1 copy at analysis/round-1/commentary/ was not touched.
- A fresh baseline was prepared from pass-003 with `commentary.py prepare`.
  - It has 1315 scaffold entries.
  - The tools were carried over into commentary/tools/.
  - All round-1 and round-2 annotations still match by bank:CPU address and physical
    offset.
  - Round-3 changes are in tools/notes3.txt, plus targeted edits in notes.txt and
    notes2.txt.
- **L-020 fragments.** These are 12 bytes of Static code:
  - AnimateTourRoute_DeadEndIf (phys 4964)
  - PowerMeterStep_DeadEndIf (phys 84584)
  - PowerMeterRise_DeadEndIf (phys 84607)
  - TextPrinterStep_DeadEndIf (phys 117621)

  Each one is commented as an ENDIF jump that nothing reaches. None is claimed to
  execute. Each has a unit record in review.json. Their targets now have proper names
  (`*_Return`) and no longer use auto `Sub_*` names. The stale unknown-gap notes were
  removed.
- **L-019 split.** The 88856+32 region note was rewritten. It holds (X, Y) pairs with
  no reader found. RackLayouts (88888+624) and the string framing (89512+5) keep their
  labels and notes.
- **L-021 fix.** NameEntryInput returns the letter number (A = 1), not the grid code.
  The following were corrected:
  - the comments for NameEntryInput, NameEntry, PasswordEntry, CheckSpecialPassword
    and DecodePassword;
  - the notes for GridLetterMetatiles, PasswordCharValue and SpecialPasswords.

  The SpecialPasswords note now gives its three consonant-only words, citing SC-016.
  No "grid code" wording remains in the source.
- **11 unknown bytes.** These are at phys 6173+1, 51444+3, 85771+5, 117501+1 and
  118162+1. Each has an honest note in the source: its decode-shaped reading, the fact
  that no reference was found, and a pointer to the round-2 redirect rejection review.
- **Backlog.** The whole source was checked against the round-2 standard. There are
  0 unnamed routine entries and no raw RAM operands. Every region and gap has a note.
  BallsTouching and VoiceTestScreen got extra block comments.

## Verification
- `commentary.py verify` returned ok=true and byte_identical=true. The result is in
  commentary/verification.json.
- The reading copy is refreshed at commentary/reading-after.asm.
- review.json has 1315 of 1315 entries reviewed and 1004 units. editorial_audit is
  still pending.

## Unresolved meanings
These are in commentary/uncertainties.md:
- the game meaning of the CPU shot variants;
- the MatchGames and RoundState encodings;
- the multiple uses of $CC;
- the picture in PasswordIconGrid;
- the routines that have no caller;
- the (X, Y) pairs at 88856;
- the unexplained byte at bank 1 $AD28;
- NameEntry's apparently unreachable `$25 -> '.'` path;
- the 11 unknown bytes.

## Leads
- No new leads were produced.
- L-021 was applied and is marked verified.
- L-017 got a commentary note.
- A pass record was appended to analysis/leads.json.

## Links
- commentary/source/game.asm
- commentary/reading-after.asm
- commentary/review.json
- commentary/progress.md
- commentary/uncertainties.md
- commentary/tools/ (build.py, notes.txt, notes2.txt, notes3.txt, review.py)
- analysis/round-2/commentary/ (the preserved round-2 phase)
