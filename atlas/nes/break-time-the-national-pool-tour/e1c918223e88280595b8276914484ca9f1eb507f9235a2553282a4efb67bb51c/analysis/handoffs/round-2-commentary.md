# Round 2 commentary handoff

Status: **COMPLETE** for current-source readability on checkpoint pass-002.
Classification is unchanged at 99.99% (163,833 of 163,856 bytes; 23 unknown, lead
L-017) and is reported separately. The final editorial audit and archive were not
run.

## Scope
- The map changed between pass-001 and pass-002. The full round-1 commentary is
  preserved, untouched, under analysis/round-1/commentary/. This includes source,
  tools, notes, review.json, the reading copies and the verification.
- A fresh baseline was prepared from pass-002 with `commentary.py prepare`. Its
  before/ directory is frozen. It has 1313 scaffold entries: 720 Code, 435 Data,
  140 Text, 8 Padding, 1 Image and 9 Unknown.
- Round-1 annotations were carried forward through tools/notes.txt. They are keyed
  by bank:CPU instruction address and physical offset, and all of them still
  matched. Round-2 additions live in tools/notes2.txt. Only stale or contradicted
  records were changed; commentary/progress.md lists them.
- Covered:
  - all new code, about 2.4 KiB;
  - all new data, about 46 KiB in 344 regions;
  - the round-1 backlog of in-routine comments;
  - the 23 unknown bytes.

## Readability assessment
- **Routines:** there are 491 routine entries, none with an address-only name, and
  every one has a header.
- **In-routine comments:** every routine with 25 or more instructions and 3 or
  more branches has block comments, and so do the large linear routines. In total
  there are 378 block-comment lines and 204 end-of-line comments.
- **Operands:** no raw RAM operand remains. FarCall targets, slot tables, string,
  screen and palette pointers, shot-type pointers and the rack-layout pointer are
  all symbolic.
- **Data:**
  - Every Data region has a format/role note. The note is either hand-written or
    chosen from the region's evidence, and it names the reading routines for
    tables classified from what reads them.
  - There are 260 data labels. Examples: VoiceSample00-13, Song01-12, Sfx16-26,
    the Screen_* menus, PasswordAlphabet and RackLayouts.
- **New understanding:**
  - The tour password system: encode, decode and three special passwords that open
    the sound test, ending and voice test.
  - The GAME SET and BREAK SHOT banners.
  - The CPU spin and scripted-button phases.
  - Sound command 7, a four-channel sync.
  - A hidden CPU handicap button combination.
  - Several leftover or debug routines that nothing calls.
- **Round-1 defects fixed:**
  - Bank 8 slot numbers $8024-$802D were off by one slot.
  - AnimateHudBars is renamed ShowBreakShotBanner.
  - Bank 0 zero-page reuse was mislabelled with title variables.
  - The voice-bank $C006 call resolved to the wrong bank.
  - The VoiceTable symbol values were wrong.
  - MainLoop_PracticeEnd is renamed MainLoop_TourLost.

## Verification
- `commentary.py verify` returned ok=true and byte_identical=true; the result is in
  commentary/verification.json.
- The reading copy is refreshed at commentary/reading-after.asm.
- review.json has all 1313 scaffold entries reviewed, plus 998 units.
  editorial_audit is still pending.

## Unresolved meanings
These are listed in commentary/uncertainties.md:
- the game meaning of the CPU shot variants;
- the MatchGames and RoundState encodings;
- $CC's triple use;
- the letters of the special passwords, which are kept as grid codes;
- the picture in PasswordIconGrid;
- the roles of the routines nothing calls;
- a possible slip in CollideHeadOnA, where the high byte is subtracted twice;
- 16 unreferenced (X, Y) pairs at phys 88856;
- one byte at bank 1 $AD28.

## Structural questions and leads
- **L-018 (new, runtime check):** verify the password round-trip and the special
  passwords with a bounded run along the SC-015 route.
- **L-019 (new, metadata):** the region at phys 88856..89517 has a string-framing
  rationale, but it actually holds unreferenced pairs, RackLayouts and a string
  header. A split would be optional and would not change any bytes.
- **L-017 (updated):** the 23 unknown bytes each have a note in the source. None is
  claimed as code.

## Remaining gaps
- No readability gaps remain in the currently classified source. Short, straight-
  line routines rely on their headers.
- Whole-ROM classification still leaves 23 bytes unknown (L-017).
- The final editorial audit and archive are reserved for final acceptance.

## Links
- commentary/source/game.asm
- commentary/reading-after.asm
- commentary/review.json
- commentary/progress.md
- commentary/uncertainties.md
- commentary/verification.json
- commentary/tools/ (build.py, notes.txt, notes2.txt, review.py, view.py)
- analysis/round-1/commentary/ (preserved round-1 phase)
- analysis/leads.json (L-017 updated; L-018 and L-019 added; pass record appended)
- analysis/gameplay.md (round-2 notes appended)
