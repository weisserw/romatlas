# Handoff: round 4, phase 2 (commentary)

**Status:** the current-source readability gate is met for all classified source on the
round-4 map. A fresh byte-identical verification passed and the reading copy is refreshed.
The final editorial audit has not been accepted: review.json `editorial_audit` stays pending
for the final-acceptance step. Whole-ROM classification (in the map) is 393,232 of 393,232
bytes, 0 unknown. This pass did not change any classification.

## Checkpoint
- Map: maps/round4-disassembly.json (= maps/current.json = analysis/state.json, sha256
  e5ad3b69...). input/game.nes is unchanged (sha256 bd987bf8...). The toolkit was not modified.
- The round-3 commentary is preserved in full in analysis/round-3/commentary/ (checked
  identical with `diff -r` before the old directory was removed). Round-1 and round-2 copies
  are untouched.
- Fresh baseline from `commentary.py prepare` (2,213 scaffold entries); commentary/before/ is frozen.
- Source: commentary/source/game.asm (sha256 c823307d...).
- New tool: commentary/tools/merge_r4.py (carry-over by physical range). Round-3 specs and tools
  were carried. leads.json was backed up to analysis/round4/leads.before-commentary.json.

## Verification
- `commentary.py verify`: ok, byte_identical true. This was the final run after the last edit.
- `commentary.py read`: commentary/reading-after.asm refreshed.
- `commentary.py audit` was run for information only. It stops only at "final editorial
  audit is pending", as intended. This is not an accepted audit.

## Scope and readability
- **Carried over:** 2,208 unchanged blocks, verbatim by physical range.
- **Region 1985** is now five documented Data blocks:
  - StaminaVarPtrs and StaminaLimitPtrs are `.word` pointer lists with symbolic targets.
  - The three tables have format, bound, index and consumer notes.
  - Old names D_B00_919B/919C/919F/91A0 now map to StaminaVarPtrs(+1) and StaminaLimitPtrs(+1).
- **L-020:** PitcherStintMark is renamed PitcherStamina at every use. The RAM note and the
  LaunchPitchedBall, CpuPitcherDecision and PlayState_22 docs were rewritten:
  - stamina is decremented once per pitch;
  - relief happens after about 40-120 pitches, and it is reachable (shown only artificially);
  - Gravity doubling happens when stamina >= RandomA.
  The "unreachable" notes were removed.
- **L-022:** the password roll range is now 0-25 and the encoder range 0-24. The CONTINUE
  real-input visit is noted.
- **review.json:** regenerated with 2,213 of 2,213 entries reviewed and 1,157 units (5 new).
  Compared with round 3, only the new entries and the 4 edited routines differ.

## Findings
- Higher limit tables mean earlier relief. Table 2 gives the earliest relief.
- Letter 25 can be entered but the encoder never produces it.

## Leads
- **Consumed:** L-020 and L-022 (analysis/leads.json, with a round-4 commentary pass entry).
- **Produced:** none.

## Unresolved questions
See commentary/uncertainties.md, round-4 section:
- Natural stamina relief has not been observed.
- The stamina effect on speed and Gravity is static evidence only.
- The pitch-type byte-5 domain has not been enumerated.
- How decode handles letter 25 is not known.
- Earlier round-3 items are unchanged.

## Deferred work
- Final editorial acceptance: `commentary.py audit`, with editorial_audit reviewed.
- Toolkit: no new issues. TK-001 to TK-005 are unchanged.
