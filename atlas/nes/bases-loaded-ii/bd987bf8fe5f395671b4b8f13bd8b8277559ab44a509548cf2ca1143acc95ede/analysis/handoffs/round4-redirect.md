# Handoff: round 4, phase 3 (redirect), attempt r4-redirect-a

## Scope
- Checkpoint: maps/round4-disassembly.json, 0 unknown bytes. The map was not changed.
- input/game.nes is unchanged and the toolkit was not modified.
- Research is in analysis/redirect/r4-redirect-a/:
  - findings.json;
  - tools/letter25_model.py and tools/byte5_domain.py;
  - experiments/y1_letter25.py, with y1_letter25.json and y1.log;
  - byte5-domain.json and letter25-model.json;
  - backups of leads and scenarios.
- Round-3 work was not repeated. This pass checked the round-4 open items for cheap tests that could tell hypotheses apart.

## Findings
- **G1: how letter 25 decodes (resolved).**
  - Method: a static model taken from the bank-4 encode/decode routines. It round-trips 26,304 encoder outputs and reproduces the r3 real-input result. A real-input runtime check followed (4 fresh boots, no RAM edits), and all 4 results matched the model.
  - Positions 1-5: 25 behaves the same as 0, because the mod-25 checksum is the same for every key byte.
  - Check letter (position 6): 25 is always rejected, because the remainder is at most 24.
  - Key letter (position 0): 25 is a working 26th key. Passwords built by hand with it are accepted.
- **G2: the byte-5 domain is 0..2 (static).**
  - The PitcherRecord bytes 0-7 come only from BuildPitcherRecord (team pitcher tables) and from the demo copy (D_B02_FC2D).
  - Across all 12 teams x 8 pitchers, the values are 0/1/2 (36/48/12), with exactly one type-2 pitcher per team. The demo pitcher is 0.
  - StaminaLimitPtrs has 3 entries, so it covers every value.
  - Caveat: computed pointers to $04A3/$04B3 were not excluded, and the EditTeam < 12 bound rests on the password check and on the 12 valid pointers.
- **G3: bank-2 pitcher data layout (new lead L-023, ready).**
  - Pointer table $FBFD: 12 words at physical 48141..48164.
  - The first target equals the end of the table, and the targets step by 64.
  - The 12 x 64-byte tables (8 pitchers x 8 bytes) end exactly where code region 508 starts (physical 48933).
  - Data stays Data, so no byte counts change.

## Rejection / exhaustion review (evidence-policy.md)
- **Classification contradictions:** none found. Low-confidence code was re-checked in r3 (no references), and no new evidence since then reopens it.
- **Natural stamina relief:** deferred. It needs a long unpatched game, roughly 40-120 CPU pitches by one pitcher. The mechanics are already shown (decrement observed; relief shown artificially). The result could not change classification, and the commentary already states it.
- **L-018 (debug code):** rejected for this cycle. There are still no references, and no natural route exists to test. An artificial jump would only show straight-line mechanics.
- **L-005:** deferred. The only open case is a far call at NMI time while bank 14 is mapped, which has no classification impact.
- **L-004:** advanced by G3. The other generic blocks are meaning-only questions with no consumer lead in hand.
- **L-009:** the password side is now confirmed at runtime. The voice and CPU-pad notes remain static readings, with diminished prospect.
- **Remaining questions:** all are semantics or need long natural play. There is no promising avenue left that could change classification.

## Ready leads
- **L-023:** label/split of physical 48109..48932 into:
  - 32 bytes of generic data;
  - TeamPitcherPtrs (24 bytes, words);
  - TeamPitcherTables (768 bytes).

  Then add commentary: byte 5 is the pitch type and stamina selector, byte 6 is the ERA-style figure, and D_B02_FC2D = team 0 pitcher 3.
- **Commentary-only:**
  - G1 notes on PasswordDecode, PasswordChecksum and PasswordDigits.
  - G2 notes on LaunchPitchedBall and CpuPitcherDecision.
  - Close the two round-4 uncertainty bullets (letter 25 and byte 5).

## Milestones
- Added MS-password-letter25 (visited, real input). No other milestone changes.

## Recommendation
- Do a commentary-only update, folding in the optional small L-023 label/split. This does not need a full disassembly research cycle.
- Then proceed to final acceptance.
- If L-023 is skipped, only the commentary notes are needed. The classification is already correct.
