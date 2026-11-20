# Round 4 redirect handoff (attempt r04-a1): TERMINAL (incomplete)

- Input checkpoint: pass-004 (maps/current.json == maps/pass-004.json == analysis/state.json; unchanged).
  262,155 / 262,160 classified; 5 unknown bytes, all L-027. Proposed: 0 bytes. Accepted: n/a.
- Research only. ROM, map, state, source unchanged; no server started. Attempt dir:
  analysis/redirect/r04-a1/ (notes.md = evidence; survey.py/.json/.txt, detail.py/.txt,
  denominators.py/.txt, twins.py/.txt; leads.before.json).
- Ready leads: none. L-027 updated (blocked). Residual: b0 $8D92 (phys 0x00DA2), b4 $B979 (0x23989),
  b5 $ACFA (0x2AD0A), b6 $9745 (0x31755), b6 $EFE5 (0x36FF5); each one byte $60.
  CPU $8000-$FFFF of 32 KiB AxROM bank b, prg_banks [4b..4b+3].

## Q-R4-1 results (combined-constraint test)
| Part | Finding | Artifact |
|---|---|---|
| (a) orphan bytes | 1,834 terminals; 293 not followed by a label/static target, almost all referenced tables, vector/pointer-reached code, structured tier-B fragments or $FF fill. Lone unreferenced orphans: 5 x $60 (unknown), $FF (b5 $8093, Data 0.8), `8B 8E` (b6 $8C13, Data 0.8), `8D` (b6 $FE2C, Data 0.8). $60 after a terminal: 48 shared exit rts (branch targets), 3 first bytes of tables (b0 $A86E, b2 $FC09, b5 $FF2C), 5 unknowns. | survey.txt, detail.txt, denominators.txt |
| (b) other padding | Yes: $FF. 17 uniform $FF Data runs between code, incl. mid-bank b2 $F131 (15), b0 $8030 (3), b6 $801E (18), and a lone $FF after rts at b5 $8093. No $EA; $00 runs only between data. | notes.md (b) |
| (c) habit analogues | None. No classified unreferenced lone rts in the ROM (0 of 1,829 other terminals). b6 $9745: 8 other identical `stx $0682 / jmp LoaderCommonTail` exits in bank 6 have no trailing $60. b6 $EFE5: twin BgObj19 (`lda #$F0 / bne $F00C` then script $F003) has none. No ROM copies of surrounding bytes. Accepted dead-code analogues (L-026 tier B `jmp RemoveObj` x5, `sta ObjFrame,x / rts` idiom, L-028 OAM-DMA prefix) each have operand/idiom structure. | twins.txt, notes.md (c) |
| (d) neighbour extents | $B979 excluded from RamRoutineTemplate_B4 (copy Y 0..$0C from $B97A); $9745 excluded from StageNextTables (`cpy #$07 / bcc` guard, bases $973F/$9744, no index-6 slot); $EFE5 excluded from the script (pointer $EFE6, forward reads); $8D92 and $ACFA have code on both sides. These exclude membership in a known structure, not an isolated stale byte. | notes.md (d) |

## Rejection review (updated; supersedes round-03 table for these premises)
- Candidate claim: each lone $60 is a dead `rts` (Code, Static, reachability none known).
- Decisive evidence: survey of all terminals and orphan bytes (survey.txt, denominators.txt), uniform-run
  scan, twin search (twins.txt), neighbour consumers (source at $9723, $B951, $EFD1, StepBgObjScript).
- What it rules out: membership in each known neighbouring table/script/template; fall-through
  execution; any direct/absolute reference; a codebase-wide "dead rts after terminal" authoring
  habit; $60 as this ROM's fill convention (fill is $FF).
- Independent constraints considered together: terminal predecessor + classified successor entry +
  no reference + value $60 + ROM-wide fill convention + absence of a habit + local twins.
  "Uniform $60 pattern" is a restatement of the value fact, not an additional constraint.
- Strongest competing interpretation: an unread stale/remnant byte (leftover of an older build).
  This class is real here: b6 $8C13 `8B 8E`, b6 $FE2C `8D`, b6 $FE24 truncated idiom. Such a byte
  could be an old rts or an old data value ($60 begins three referenced tables).
- Remaining reason insufficient: the combination shows the $60s are anomalies, not a convention;
  nothing ties them to code beyond the opcode value. Per evidence-policy.md an unreferenced return
  opcode lacks sufficient structure on its own, and no added constraint discriminates.
  Corrected premise of Q-R4-1: "no other padding exists" is false ($FF fill exists, including a lone
  $FF), and "orphans are systematically $60" is false.
- Reopen only with: a new reference or consumer for one of the five addresses, or a twin/copy of
  the surrounding routine where the $60 has a role.

## Other avenues
- None actionable: all other bytes classified; commentary readability complete (round-04-commentary);
  gameplay milestones visited (analysis/scenarios.json; remaining "attempted" entries are invalid stage ids).
- Housekeeping: scratch 3.2 MB, ~4.1 GB free. No new toolkit issues.
