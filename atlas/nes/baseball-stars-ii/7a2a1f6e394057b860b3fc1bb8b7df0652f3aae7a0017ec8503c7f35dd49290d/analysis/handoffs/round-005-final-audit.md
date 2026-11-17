# Round 5 final acceptance: editorial audit (2026-11-17)

## Result
- `commentary.py audit`: **PASS**. It reports ok, byte_identical true, 4,111 reviewed regions, 3,453 reviewed units
  and editorial_audit true, and writes commentary/audit.json. The audit also refreshed reading-after.asm.
- A separate `commentary.py verify` run after the audit gave ok and byte_identical true.
- Nothing was touched in input/, maps/, analysis/state.json, analysis/round-004/ or the toolkit. Scratch scripts are
  in /tmp/fa.

## What was reviewed
This was a targeted review, not a whole-source reread, and earlier review credit was kept.
- Mechanical checks over all of review.json:
  - Every source location exists.
  - Every unit name appears on one of its source lines.
  - Every entry's first location sits on its region header. The only exception is CpuVectors, which has no header
    line.
  - The scaffold (id, offset, length, kind, phase1) and every unit's offset and length are unchanged against the
    pre-edit copy.
- Round-5 additions:
  - Fragment units ("Part of X") were sampled.
  - About 10 of the 105 new block comments were checked against their code, for example the AcquireConfirm
    YES/NO handlers.
  - All 14 cross-pair call notes in banks 30/31 were checked. Each target address was recomputed from the bank
    source, and all of them matched.
- Pitcher/batter card swap: the code supports it. ReleasePickPlayer (bank 24) and PlayerSelectPickPlayer (bank 25)
  store ObjTask+1 = 0 for places 0-5, and DrawPlayerCard jumps to DrawPitcherCard when that value is 0. Every
  reference was checked: labels, comments, banks 18/20, and the unit names and summaries in review.json.
- Stale and placeholder text: searched for TODO, placeholders, "Purpose not yet", "unknown"/"unclassified" used
  as a current status, references to removed renderer tools, and pass-004 status. The only problem found was the
  status wording noted under the fixes.
- Address-only labels and RAM aliases: no Loc/Tbl/Unk labels remain. Every RAM address with two names in ram.inc
  is documented as a bank-specific overlay.

## Defects found and fixed
1. **Contradictory RAM role (3 source sites, 7 units).** PendingPitcherChange = 2 was described as
   "computer-vs-computer play" in Mode14TimeOut and TimeOutChooseSide, and as "set when the computer pinch-hit"
   in ram.inc.
   - The code says otherwise. PinchHitterApply, the pinch-runner path and CpuSubHitter set it to 1.
     Mode9HalfInningEnd increments it to 2, and bank 9 then goes straight to time-out mode 14.
   - TimeOutChooseSide fakes A for side 1 (SidePressed+1 = BTN_A), not for side 0.
   - The Mode14TimeOut unit summary had also been cut off mid-word ("BaseOrSu").
2. **Labels that did not match the swap (2).**
   - ScreenPlayerCard is now ScreenBatterCard. It is the batter path of DrawPlayerCard, ScreenDataTable word 78.
   - MetatilesPitcherCard is now MetatilesPlayerCard, because both card screens use it.
   - The comments and unit names were updated to match.
3. **Summaries cut off mid-word (about 130 units).** Examples are DefenseFrame ("ball fiel"), StadiumPickerInput,
   Mode2SetupField, GameModeHandlers and many sound-driver units. They were restored from the full source
   comment.
4. **Fragment summaries that hid a caveat or repeated text (904 units).** Fragment units copied the whole routine
   comment, and some copies stopped before a "Tentative:" caveat, for example FarDrawPlayerPicture. They now read
   "Part of X (kind fragment, CPU range, bank); the routine is explained by the comment above its label (second
   source location)", followed by their own branch labels and notes. The 337 short routine texts (100 characters
   or fewer) were left as they were.
5. **Boilerplate entry summaries (4,078).** Generic "Code read in rounds 1-3 ... pass-004 baseline" and "Data kept
   as raw bytes ..." text was replaced.
   - Code entries now give the CPU range, the bank and the list of unit names.
   - Data entries now give the CPU range, the bank and the region's own source note. That note is the header
     comment, the trailing directive comment or the inline-parameter JSR. Nothing new was invented.
6. **Hyphenation artifacts (33),** such as "human- controlled". Most were already present in round-4 text.
7. **Cross-pair notes.** The first jsr $8274 in bank 30 (@walk_runner) is now confirmed as pair 6: it is selected
   at @processResult, and ForceRunnersOnWalk restores it. The EndOfPlay call note now names the callers it checked
   and the ones it did not.
8. **Smaller fixes.**
   - ram.inc had doubled "banks N: bank N:" prefixes (4) and "banks N:" written for a single bank (14).
   - Two fallback prior-ambiguity notes used the present tense ("stay unknown"). They now use the past tense.

## Residual limitations
- Many raw RAM operands keep their addresses because their roles are unknown, for example $04FB, $057F, $06A1,
  $074F and $0711. Some table operands are also still raw, such as the bank 1 $B240/$B340 speed tables, and the
  local label @table_b340 is named after that address.
- ScreenRecordNN labels are index names for screen records that are reached only through ScreenDataTable.
- 127 global fall-through or branch labels have no comment of their own and rely on their routine headers.
- Short data-unit summaries, such as "Six spaces.", are terse but accurate.
- EndOfPlay's jsr $8274 is still "probably pair 6" on the FinishPlayRedraw and HomeRunTrot paths.
- The audit checks structure only. It cannot judge prose quality, which was covered by sampling.
- **The 8 bytes in regions 5155-5158 (bank 2 $80DA-$80DB, $9988-$998B, $9B48; bank 9 $A165) are generic Data of
  unknown meaning. They were classified only by a user-directed Manual fallback and are not backed by observed or
  static evidence. No meaning is claimed for them.**
