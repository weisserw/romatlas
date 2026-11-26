# Round 3 redirect handoff

**Recommendation: another disassembly/commentary cycle.** Ready leads: **L-014** (provenance,
11 regions, 133 bytes) and **L-018** (commentary). L-001/L-002 are now resolved-unreachable,
and their classification is unchanged.

## Scope and identity

- Input: checkpoint **pass-003** (analysis/state.json = maps/current.json, 511 regions) and the
  round-3 commentary (byte-identical).
- ROM classification stays at 100.00% (65552/65552, 0 unknown; the unknown complement is empty).
- Proposed, not accepted: Static -> Observed for 11 code regions (133 bytes). If accepted, Code
  Observed goes 11775 -> 11908 and Static 2084 -> 1951. No boundary or kind changes.
- Not modified: map, state, commentary source, ROM, toolkit, configured binary.
- Attempt directory: analysis/redirect/r3-a1/ (README.md with the experiment tables, tools/, evidence/).
- The server (/tmp/r3a1.sock) was stopped and scratch (/tmp/r3scratch) was removed. Evidence is 4.2 MB.

## 1. Assessment of the round-2 L-001/L-002 rejection review

- **Premise problem.** The round-2 review searched for byte patterns: entry and entry-1 words, and
  #lo/#hi immediates within 16 bytes. That kind of open-world search can miss:
  - split lo/hi tables;
  - computed `pha`/`rts` pushes;
  - RAM jump vectors;
  - script opcodes;
  - debug entries.

  Its "no reference found" therefore did not justify "no selector exists". The round-2 file also
  marks operand coincidences as `true` (for example `D0 F6` = `bne`). That flag means "inside
  code", not "code-consistent".
- **Corrected method (S1, tools/roots.py, evidence/closure.json).** Instead of searching for
  references, every instruction that can transfer control to a computed address was enumerated
  over all 6665 decoded Code instructions:
  - `jmp (ind)` appears only at $F616/$FD5E/$FED6, all inside uncalled inline-argument routines;
  - there is no BRK and no JSR/JMP into RAM;
  - the one computed RTS dispatch is SoundCmdDispatch ($E17A-$E185). Its index is (cmd-$F0)*2 with
    cmd >= $F0 enforced, so the 16 entries equal the table size;
  - the only stack writes are Random's saved-A slot ($0100,X) and the sound driver's stack switch,
    which is restored;
  - there is one RTI (NMI);
  - every reachable branch or jump target is an instruction start, and no reachable instruction
    falls through into non-code.

  A split table, script opcode or debug selector would need one of these transfer forms to reach
  code, so none can.
- **Result.** The closure from RESET/IRQ $F22A, NMI $F260 and the 16 sound handlers contains 5904
  instructions. Its complement is exactly the 23 L-001/L-002 runs, 1461 bytes.
  - These bytes are statically unreachable from the vectors, assuming normal stack discipline;
    hardware faults are not considered.
  - The claim is about reachability only. Code/Static stays, supported by the round-1/2
    contract evidence (inline-argument convention; island immediates sized to the data that
    follows them).
  - Commentary may state this (L-018).

## 2. New findings

- **RNG contract (S2).**
  - Random ($F88D) has four call sites: Random(7) per throw when the ball weight differs from
    PreferredBallWeight[character] ([2,3,1,0]), Random(4) per throw at StrikeStreak >= 3, and
    Random(10) at double and turkey celebrations.
  - RandomSeed ($05E5) is zeroed only by Reset.
  - So the celebration variant depends only on the history of these calls. The offline model
    (tools/rng.py) reproduces snap_f10_last's seed (19 calls) and every seed seen in E14.
- **E14 (natural).**
  - Start: snap_lane1, with character 0 and weight 0 (a weight mismatch).
  - Inputs: frame 1 two gutters, then strikes in frames 2 and 3.
  - The model predicted Random(10) = 7, which selects @doubleB → CelebrationTilesA. Both ran:
    $BAA6 (67 bytes) and $B847 (6 bytes).
- **E15 (natural).** 3P 10th frames, starting from r2-a1 e11_frame10_start.json:
  - the patterns were X 9 /, 7 / X and 7 / -, with totals 289/277/257;
  - a traced replay of the kept inputs reproduced every ball;
  - it covered ScoreBall/ScoreFrames/WriteTenthFrameMarks spare/miss branches (55 bytes; $A845 4/5).
- **E16 (natural).** A power 6 throw → ReleaseBall SFX_ROLL_SLOW $86C7.
- **S3 (CHR).** On the scorecard glyph set (banks 1/2, $1000 half):
  - $42 = "-" and $43 = "G";
  - $44 = spare box and $45 = strike box;
  - E15's scorecard shows "-" for the third-ball miss.

  This resolves the open uncertainty about tile $43.

## Ready leads (analysis/leads.json)

- **L-014 (low, disassembly):**
  - Set Observed for ids 29, 133, 381, 383, 385, 145, 388, 389, 391, 395 and 241.
  - Evidence: r3-a1/evidence/gain_r3.json and the three coverage files.
  - Re-decode instruction starts and check them against the coverage. Region 379 stays Static.
- **L-018 (low, commentary):**
  - add the RNG/celebration contract to StrikeCelebration, Random and the hook code;
  - add the tile $42/$43 glyphs;
  - reword the L-001/L-002 comments and the header to state the closure result;
  - update uncertainties.md.

## Remaining residuals and deferred avenues

- **490 bytes of reachable Static branches** (if L-014 is accepted). These are deferred because
  they give provenance only and the evidence lacks a trigger.
  - NMI/library jobs gated by AttrRowPending, VramReadMode, FillPending, SpriteCycleMode 1-$EF and
    WaitSecondCount. No reachable direct store opens these gates (gate_writers.json); they could
    only open from the uncalled library or an unresolved indexed store.
  - Page-crossing carries ($AAF7, $AB07, $BDEB, $E267). These need specific pointer or total
    values.
  - Fade-delay, sound envelope and DrawObject $FD/clipping variants.
  - The left-gutter test $8A2D. Natural gutters went right, and left aim seems clamped above
    $C0. This is not verified.
  - $A845's `jmp`: it needs a third ball after an open 10th, which the frame logic never allows.
- **L-010 (open, low):** per-record splitting of sprite lists is source formatting, with no
  research question. Leave it to commentary or the final editorial audit.

## Unvisited gameplay milestones review

- The game has no levels, bosses or credits.
- Visited naturally:
  - title and menus;
  - 1-4P selection, lanes 1-5;
  - full games;
  - all strike/spare celebrations, now including CelebrationTilesA (S27);
  - 10th-frame mark patterns (S28) and the slow roll (S29);
  - results, the three award scenes, hi-score ranks 1-3, and the return to title.
- Pause is **unavailable**, not deferred: CheckPause $F7AE is outside the vector-rooted closure (S30).
- No practical milestone remains.

## Toolkit

No new issues; TK-001..TK-005 kept. Traced emulation is slow (TK-004): about 5 minutes per traced
ball. To keep traces short, outcomes were searched untraced and only the kept inputs were
replayed traced.

## Markdown files created this phase

- analysis/handoffs/round-3-redirect.md (this file)
- analysis/redirect/r3-a1/README.md
