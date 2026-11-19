# Redirect handoff - round 1, attempt r1-a (2026-11-18) - STATUS: COMPLETE, ready leads produced

## Scope and checkpoint
- Input: checkpoint pass-001 (maps/pass-001.json, 94.08% classified, 23,287 unknown bytes in
  1,090 runs) and commentary round 1 (analysis/commentary-handoff-round1.md).
- Redirect changed only research/lead records: analysis/leads.json, analysis/scenarios.json,
  analysis/gameplay.md, analysis/toolkit-issues.md (TK-003), analysis/progress.md and this
  directory. Map, state, source, ROM and toolkit untouched. No map annotation done.

## Exact unknown complement (ledger)
`unknown-ledger.json` gives every one of the 1,090 unknown runs (physical range, bank, CPU,
neighbours, closure statistics) a disposition; nothing is left unassigned:

| Lead | Kind | Bytes | What |
|---|---|---|---|
| L014 | Data | 20,204 | data-interior runs (tiers A 18,443 / A-style 759 / A2 1,002) |
| L012 | Code+Data | 1,730 + 68 | unreferenced closed routines in units 0-6/12/16/18/20/22/26/28 + 3 small tables |
| L013 | Data | 597 | consumer-anchored tables in code-adjacent runs |
| L009 | Code+Data | 276 + 89 | inline jump tables, case code, mask/threshold tables (L001 residue) |
| L010 | Code | 298 | fixed-bank library routines (L004) |
| L011 | Data | 25 | bank 31 residues: internal header tail, post-sample bytes (L006) |
| total | | 23,287 | = pass-001 unknown |

Plus L015: 109 bytes currently classified Data in unit 28 that are code (correction, no
unknown-byte change). If all are accepted: unknown 0, code +2,413 B (incl. L015).

## Findings and methods
1. **Closure test for data-interior runs (L014, reopens L002 with a different method).**
   Phase 1 left runs unknown whenever any linear decode stayed legal for 20 bytes. New test
   (`tools/gapclass.py`): for every byte of a run, follow *all* paths (both branch arms, jsr
   continuations, jmp targets); a fragment counts as possible code only if every path ends
   in rts/rti/jmp(ind) or reaches a classified instruction start, all absolute operands lie
   in RAM $0000-$07FF, PPU $2000-$2007, APU/IO $4000-$4017 or ROM $8000+ (this game never
   uses $0800-$1FFF/$5000-$7FFF; checked over all code operands), and it has >= 4
   instructions. Tier A runs (18,443 B) have no such fragment even with all official
   opcodes. A-style (759 B): fragments exist only with opcodes that never occur in the 57 KiB
   of classified code (`code-opcode-histogram.json`: 38 official opcodes unused, e.g. (zp,x)
   forms, php/plp, clv, sed). A2 (21 runs, 1,002 B): short accidental fragments in content
   inspected by hand (tile maps 36 36 36..., metasprite records, text tiles 26 27 28...,
   pointer words). No run is the target of any classified code operand or branch, no byte
   was fetched in the round-1 traces (`traces/round1-observed-contexts.json`), neighbours are
   Data/Padding. Random visual sample: pointer tables, tile/attribute runs, sound data.
2. **Code-adjacent runs.** Every code-preceded unknown run starts after an rts/jmp except the
   four inline-dispatch sites (jsr JumpTableDispatch). Each L009/L010/L012 code sub-range was
   checked by `tools/verify_props.py` (output `verify-proposals.txt`): linear decode ends
   exactly at the range end, all branch/jsr/jmp targets are classified instruction starts or
   co-proposed starts, no fall-off into data, operands valid. All OK.
3. **Inline tables (L009)** are closed by independent endpoints: unit 26 tables end at their
   own first in-gap target ($995D observed, $9A78); unit 20 table at the lowest in-gap target
   $93E3; unit 28 9-word table ends at $9746, the absolute operand of `and $9746,x`.
4. **Fixed-bank library (L010):** $CA15 VRAM string writer, $CBE1 16x16 multiply, $CC93 8/8,
   $CCAC 16/16, $CCD5 24/24 divides, $CD3A integer square root - closed, stack-balanced,
   same Arg $10-$1F convention as classified neighbours ($CD12, $CC80).
5. **Rule-(a) data overclaim (L015, new finding).** `tools/tails.py` checked every phase-1
   "base operand" data region for a closed code suffix: exactly two in unit 28. $ACC8 catcher
   pose table index is bounded Y <= 19 by $AC8B/$ACA2/$ACC2, so $ACDC-$ACEB is code ($ACE0
   called from the L012 helpers $9BEC/$9C12); $A99C-$A9F8 is a closed routine pair calling
   the classified multiply $CD12 and the L010 divide $CCAC. These give L010 a static caller.
6. **Cartridge header (L011):** $FFF0-$FFF1 = $0EB1 = 16-bit sum of all PRG bytes except
   those two; $FFF2-$FFF3 = $0D6A = sum of CHR ROM (both verified exactly); $FFF7 = $0D = 13 =
   length-1 of "BASES LOADED 4".
7. **L003:** $84D8 is already Data (512 B, 11 refs); the scan of every $3A/$3B writer found
   computed far calls only via U26_FieldUnitTable and unit 28 $8926 (units $0A/$0C). Closed.
8. **L007:** FieldUnitIndex holds only 0/2 (units 0/4); no other selector for units 2/6.
   Natural reachability rejected for now (static classification unaffected).

## Gameplay progression (scenarios.json S09a-S09e, S10)
- Season RAM contract confirmed; artificial shortcuts (exact patches in scenarios.json):
  GAME OVER (S09a), pennant LEAGUE CHAMPION (S09b), Super Series password screen
  "000R6CB640" (S09c), SUPER SERIES CHAMPION -> title = US terminal screen (S09d), orphaned
  Japanese sub-mode-19 scene (S09e; 40 traced frames executed no unknown byte).
- 2P exhibition visited by natural input on both ports (S10).
- Evidence screenshots/scripts/outputs: `evidence/` (snapshots were scratch and deleted).

## Ready leads (for the next disassembly pass)
L009, L010, L011, L012, L013, L014, L015 (details/recipes in analysis/leads.json; ranges in
`tools/proposals.py`, `tools/make_ledger.py`, `unknown-ledger.json`). Suggested order: L015
first (split two Data regions), then L009/L010/L012 code, then L011/L013/L014 data.

## Deferred / rejected avenues
- L007 (units 2/6 streamer reachability): negative evidence (selector domain {0,2}); no
  further method without a new selector writer.
- Sub-mode 19 natural setter: byte search found no `lda/ldx #$13` feeding $2F/SetGameMode;
  the likely link is the bypassed unit-18 code ($B70C jmp $B727). Reachability only - no
  classification depends on it.
- Natural password entry of "000R6CB640": untested; would turn S09c into a natural route but
  does not affect classification (diminished prospect). Password character set not decoded.
- Specific semantics of L014 data (sound envelopes, team records, metasprites): not needed
  for classification; commentary can refine labels later.
- L008: superseded by the final-scoreboard RAM transition.

## Residual risk / review notes for disassembly
- L014 rests on a negative (no closed code fragment) plus data context; reviewers should
  spot-check A-style and A2 runs, where the argument is weakest (1,761 B).
- L012 routines have no caller (except via L015); classify as Static code with reachability
  unproven - do not call them dead.
- Scratch left: a few small PNGs in scratch/ (deletion was refused by the sandbox permission
  check); disposable.

## Another disassembly cycle?
Yes. The ready leads cover the whole unknown complement (23,287 B) plus a 109-byte
correction; a disassembly pass can validate and annotate them, then commentary and final
editorial audit follow.
