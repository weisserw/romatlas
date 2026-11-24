# Round 3 redirect handoff (attempt r3-a1)

Status: COMPLETE. Recommendation: TERMINAL INCOMPLETE (2 unknown bytes); proceed to final
editorial audit/acceptance.

## Scope / checkpoint
- Input: pass-003 (maps/pass-003.json == analysis/state.json, sha256 9baf6bef...e17b6, 599 regions),
  commentary round 3 COMPLETE (byte-identical). Classification 131086/131088 (100.00%, not complete).
- Changed evidence since r2-a1 only: round-3 disassembly (L2-02 provenance/splits) and round-3
  commentary (prose corrections, renames, uncertainties.md round-3 section).
- No map/state/source/toolkit changes; no runtime runs. input/game.nes unchanged.
- Notes: analysis/redirect/r3-a1/notes.md (leads snapshot leads-before.json).

## Findings
- Round-3 changes touch neither residual byte nor any orphan routine/unread data; commentary
  corrections describe already-observed code and add no caller, selector, pointer or reader.
- New method (split-table blind spot of earlier word/immediate scans): decoded all 7 pha/pha/rts
  dispatch tables and every jmp(vector) writer. No target equals $A019/$A0EC/$E4B3/$E585,
  $B6B0 or $B685 (or target-1). Negative; confirms prior premises.
- BottomChain2000_EntryLo (RTS byte doubling as table entry 0) checked as a possible convention
  for $B6B0: not analogous (entered + table-read; $B6B0 is neither).

## Ready leads
None. leads.json: pass record added (round 3 redirect, empty produced/ready); L2-01 attempt appended
(blocked, terminal).

## Residual unknown complement (unchanged)
| phys | bank/CPU | byte | disposition |
|---|---|---|---|
| 63168 | bank 3 $B6B0 | $60 | after `jmp PrintBigText` (PrintPlayersOption), before OptionStrings; unknown (L2-01) |
| 79509 | bank 4 $B685 | $EA | after sound data `$FF`, before CopyRamTileRoutine; unknown (L2-01) |

## Rejection review
r2-a1 review (analysis/redirect/r2-a1/notes.md, "L2-01 rejection review") reused: its premises are
unchanged (identical bytes and neighbours in pass-003, no new reference even under the split-table
decode, no ROM-wide jmp+rts/pad convention, no original listing). Code reading (dead rts/nop) and data
reading (pad/residue) predict identical behaviour; a lone valid opcode with no format membership
supports neither Code structure nor generic-data pattern evidence, and non-execution alone does not
prove data. No revised test discriminates.

## Deferred avenues
- Orphan code $A019/$A0EC, $E4B3, $E585: classified Code (Static); reachability none - no caller in
  any direct, word, immediate, split-table or indirect dispatch form. No method left.
- Bank 3 $A500-$AEFF: classified Data; reader unknown; all scenario milestones visited, no new mode
  to watch. A reader would only add a role.
- Semantic open items (uncertainties.md) do not affect classification.
