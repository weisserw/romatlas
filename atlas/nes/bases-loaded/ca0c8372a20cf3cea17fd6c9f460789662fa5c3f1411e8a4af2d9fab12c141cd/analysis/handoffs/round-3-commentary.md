# Round 3 — commentary phase handoff (2026-11-19)

Gate: met. Current-source readability is complete for ALL classified source, a fresh
byte-identical verification passed, and the reading copy is refreshed. The final editorial
audit was NOT run: editorial_audit stays pending for the final-acceptance worker.
ROM classification: 100.00% (327696 / 327696 bytes classified; 0 bytes unknown).

## Scope and method
- The map changed from pass-002 to pass-003 (maps/current.json sha256 4b3c1d85... = pass-003). The whole
  round-2 commentary/ directory was copied to analysis/round-2/commentary/ and checked identical with
  `diff -r`. Then commentary/ was removed and `commentary.py prepare` was re-run on pass-003
  (1,497 regions, byte_identical true). This is the TK-005 workaround. analysis/round-1/ was not touched.
- notes/, tools/, uncertainties.md and progress.md were carried from the round-2 copy. The notes are keyed
  by CPU address per 32 KiB bank, which is the physical range. The first rebuild was byte-identical and
  dropped no notes.
- Changed regions (only 3): bank 3 $9CAC is now Code (L-028), AnimationStepTimes is $A398+5 and
  DrawStepFigure is $A39D+31 (L-029). Notes were re-targeted:
  - UnknownByte9CAC became NullHook9CAC, with a comment giving the layout evidence, confidence 0.6 and
    unresolved reachability.
  - The DrawStepFigure label moved to $A39D.
  - AnimationStepTimes got a format note: entries 2-4 are never read, and the extent is a sibling-layout
    inference.
  - build.py now emits the generated "reachability unresolved" region note for the L-028 byte too.
- Invalidation: review.py carries a round-2 entry only when both its physical range/kind and its rendered
  source text are identical. Results: 1,475 carried, 19 same-range entries re-reviewed because their
  comments changed, 3 new. Total 1,497/1,497 reviewed with 1,887 units.

## L-031 resolution (redirect r2-a1 F3-F6) — lead set to verified
- A button, not Start (Pad1NewPress AND #$80):
  - Fixed in SceneFrame17 and SceneFrame18.
  - Also fixed in SceneFrame15, which has the same bug and was found by a button-bit scan of banks 0-3.
  - EndingStartToTitle was renamed EndingPressAToTitle.
  - CpuOpenPinchMenu is now described as a faked Start press, since it writes $10.
- L-030: the ScaleLaunchByHomeLead header now covers:
  - Its caller, bank-0 BeginBallInPlay at $9B6C, in a computer game during the top half.
  - The real effect: while the home side leads, LaunchSpeed is forced to $0400.
  - The MulU8/MulU8Z mismatch, which only matters when the game number is 0.
  - BeginBallInPlay also notes the call. The "caller not identified" text is gone.
- L-026: SceneInit17SeasonEnd and SaveGameState now explain the probable game bug. The music request is
  wiped by the RAM clear, so neither season-end exit is ever performed. The text says this was confirmed
  only in artificial runtime tests. SceneFrame17 cross-references it.
- PennantCounterA is now described as the game number in ram.inc and in every dependent text:
  - NextScheduleEntry, now explained: it picks entry (counter mod 33)/3 of the team schedule, then
    increments the counter.
  - Result panel, password routines, DrawSeasonRecord and DebugSetSeason.
  - Bank-1 CpuMenu5Pick, PlanCpuSubstitutions and CpuSubCountTable.
  - uncertainties.md has a round-3 section.

## Readability assessment
Unchanged from round 2, and good enough for porting:
- Every routine has a comment, and recurring RAM operands are symbolic.
- Record layouts and data formats are documented where they are stored.
- No automatic Sub_/Lxxxx labels remain. Data_9490 is kept deliberately and explained.
- No mechanical summaries remain, and no global code label lacks a comment (tools/nocomment.py).
- The only unlabeled blocks after RTS/JMP are the known return-address and RTS-split continuations:
  bank 0 $86E3/$86FA and bank 3 $8397/$857D.

## Verification
- `python3 /root/romatlas/scripts/commentary.py verify <workspace>`: ok, byte_identical true, ROM sha256
  ca0c8372... (commentary/verification.json).
- `read` refreshed commentary/reading-after.asm.
- input/game.nes sha256 is unchanged.

## Known editorial issues for the final audit
- Branch labels are still derived from their conditions (`@ifPennantCounterBBelow`, `@ifBelow`) where no
  game meaning is established. This was accepted in rounds 1 and 2.
- Lower-confidence classifications are explained in the source but are not proven:
  - L-024 code: 2,426 bytes at confidence 0.75 with no known entry. Each region carries a generated note.
  - NullHook9CAC: confidence 0.6.
  - About 74,961 bytes of round-1 heuristic generic data at 0.75: boundary/evidence notes only, with no
    format claim.
  - AnimationStepTimes entries 2-4 are never read.
- Some data-region headers reuse the first sentence of the phase-1 evidence. This is terse but accurate.
- Two comments cite redirect r2-a1 runtime tests by name (prg32_1 ScaleLaunchByHomeLead, prg32_3
  SceneInit17SeasonEnd). The audit may prefer plain wording.
- Open meanings, listed in commentary/uncertainties.md:
  - Scenes 3, 4 and 19 are described only mechanically.
  - Why AddError is called on the fourth bounce.
  - Port7000.
  - Voice sample meanings.
  - The CPU decision tables.
  - ScreenImageA/B.
  - The RunsTally purpose.

## Deferred work
- The final editorial audit (`commentary.py audit` with editorial_audit recorded).
- Optional renaming of branch labels by game meaning.
- Optional runtime confirmation of scene meanings, and of the season-end stall in a natural 132-game season.
- No toolkit changes were made. TK-005 (`prepare` has no later-round mode) still applies, and no new
  toolkit issues were found.
