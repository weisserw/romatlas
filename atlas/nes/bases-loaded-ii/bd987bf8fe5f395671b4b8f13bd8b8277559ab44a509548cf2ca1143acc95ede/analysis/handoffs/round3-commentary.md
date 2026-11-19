# Handoff: round 3, phase 2 (commentary)

**Status:** the current-source readability gate is met for all classified source on the
round-3 map. A fresh byte-identical verification passed and the reading copy is refreshed.
The final editorial audit has not been accepted: review.json `editorial_audit` is still
pending, reserved for the final-acceptance step. Whole-ROM classification is complete in
the map (393,232 of 393,232 bytes, 0 unknown). This pass did not change any classification.

## Checkpoint
- Map: maps/round3-disassembly.json (= maps/current.json = analysis/state.json, sha256
  bfba143a...). input/game.nes is unchanged (sha256 bd987bf8...). The toolkit was not modified.
- The round-2 commentary was preserved in full in analysis/round-2/commentary/ (source,
  specs, tools, review, progress, uncertainties, before/). The round-1 copy in
  analysis/round-1/ is untouched.
- A new baseline was made with `commentary.py prepare`. commentary/before/ is frozen.
- Source: commentary/source/game.asm (sha256 4a189a2c...).
- Specs: commentary/specs/r3-01-ram, r3-02-bio, r3-03-docs, r3-04-ram2. Older specs are
  carried for history.
- New tools:
  - tools/merge_r3.py: carry-over by physical range.
  - tools/where.py: source lookup by bank:address.
  - tools/rawuse.py: raw-operand finder.
- tools/review.py now adds continuation units and writes `game.asm:N` source locations.
  The round-2 review.json used `source/game.asm:N`, which the audit rejects.

## Verification
- `commentary.py verify`: ok, byte_identical true. This was the final run after the last edit.
- `commentary.py read`: commentary/reading-after.asm refreshed.
- `commentary.py audit` was run for information only:
  - Every inventory, review and unit check passes: 2,209 of 2,209 scaffold entries reviewed,
    1,152 named units, and 0 code regions without units.
  - It stops only at "final editorial audit is pending", as intended. This is not an
    accepted audit.

## Scope and readability
- **Carried over:** 2,200 blocks whose range and kind did not change are carried verbatim.
- **Former L-016 ranges:** the 7 ranges (b2 $D5FE and $DAC9-$DACC, b6 $CA2E, b11 $C286 and
  $C9C4, b15 $C8FE and $D9F4) are now Code blocks. Each comment says:
  - it is a stray return, unreachable as far as known, with low confidence;
  - where it sits (three sit inside routines, with a forward branch jumping over the byte);
  - for the bank-2 epilogue, that it matches the PLA/TAY/PLA/RTS epilogue of several routines.
- **Region 517 split:** ScreenLoaderStepTable is rendered as a `.word` list. The remainder,
  and its neighbours up to $C155, are documented as six 22-entry layout descriptor tables
  (Layout_PpuAddr, MetatileMap, AttrData, Palette, MetatileDefs, ChrBank). The 800-byte block
  that follows is described as palettes and metatile definitions. All screen-loader routine
  docs were rewritten.
- **L-017 tables:**
  - Old "generic data / presentation only" notes were replaced.
  - Handlers no longer say "no caller is known"; they say they are reached only through
    their table.
  - The CpuSideMode table note was corrected: code stores modes 0, 1, 2 and $40, and the
    other entries are filler.
- **L-011:**
  - Constants REC_BIO_PHYSICAL, REC_BIO_EMOTIONAL and REC_BIO_INTELLECTUAL are used at all
    17 read sites.
  - 10 routine docs were rewritten. CpuBat_PitchReleased and CpuBat_NoSwing had no doc and
    now have one.
  - PlayState_27 was corrected: the walk message reads the batter's value, not the pitcher's.
- **L-018:** the source notes now cite the redirect's negative reference search.
- **RAM:** raw RAM operands went from 289 to 18. 82 RAM operand mappings were added (26 new symbols, the rest Name+n forms),
  for example PitchCurve, PredVelX, PitchTargetX/Y/Z, BatterRecordsA, PitcherWildness and
  LoaderPpuAddr. DemoFlag was renamed PitcherStintMark because the old name contradicted
  the code.

## Findings
- **CPU relief by stamina:** PitcherStintMark ($06C4/$06C5) is only ever set to $FF by direct
  stores. That makes CpuPitcherDecision's stamina relief branch look unreachable (new lead
  L-020).
- **$06C7 (PitcherWildness):** +1 per ball, -2 per strike, plus the remaining balls on a hit
  batter. The CPU relieves the pitcher at 4 or more with an 80% roll.
- **Screen layouts:** the bank-3 screen layouts use 2x2 metatile maps of 16 x 15 cells. Each
  frame streams one $80-byte packet.

## Leads
- **Consumed:** L-011.
- **Updated:** L-018.
- **Produced:** L-020 (semantics only, low priority).
- All are recorded in analysis/leads.json.

## Unresolved questions
These are in commentary/uncertainties.md, round-3 section:
- The biorhythm effect is static evidence only.
- PitcherStintMark writes through indirect addressing were not excluded.
- ReleaseOriginX/Y and the PitcherSprY pair are only mechanical readings.
- The ScreenLoadId range is not traced.
- 14 raw RAM addresses remain, all single-purpose stores.
- Earlier front-end, PlayResult and category questions remain.

## Deferred work
- Final editorial acceptance (`commentary.py audit` with editorial_audit reviewed).
- Optional runtime checks for L-011, L-018 and L-020.
- Toolkit: no new issues. TK-001 to TK-005 are unchanged.
