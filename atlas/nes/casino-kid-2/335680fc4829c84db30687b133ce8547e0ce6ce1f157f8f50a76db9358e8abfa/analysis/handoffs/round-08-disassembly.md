# Round 8 / disassembly handoff

## Scope
One disassembly pass on pass-007. It applied the two ready redirect leads from r07-a1, R7-L041 and R7-L042.
- Changes are provenance, evidence and confidence only. Partially executed Code regions were split at instruction starts.
- There are 0 classification-byte, 0 kind and 0 bank-context changes.
- analysis/state.json was restored. The baseline was not re-created.
- input/game.nes is unchanged, and there were no toolkit edits.
- I used the binary named in workspace.json through a temporary socket bridge. Scratch /root/rascratch-r8d has been
  removed and my server stopped. No other task's process was touched.
- No gameplay. Neither lead needed fresh runtime evidence; the saved r07 coverage was reused. S01-S17 already cover the
  milestones, so scenarios.json is unchanged.

## Checkpoint
**pass-008**:
- maps/pass-008.json (sha256 569ab31cb21db29e91ce4d6505dda53cccad47a07eaf286aeb0fd044bc57bbdb)
- generated/pass-008
- analysis/pass-008-verification.json

## Verification
- map.validate ok.
- analysis/state.json == maps/current.json == maps/pass-008.json (same sha256).
- `run.py checkpoint pass-008`: ok=true, byte_identical=true.
- Byte-wise comparison with pass-007:
  - no overlaps;
  - identical Code, Data and Padding byte sets;
  - identical per-byte bank context;
  - all 4,431 coverage opcodes lie in Observed code.
- The live apply log is identical to the `--dry` simulation (176 operations).

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

Per bank:

| Bank | Code | of which Observed | Data | Padding | Unknown |
|---|---|---|---|---|---|
| header | 0 | 0 | 16 | 0 | 0 |
| b0 | 3 | 0 | 16090 | 291 | 0 |
| b1 | 3 | 0 | 16142 | 239 | 0 |
| b2 | 1546 | 1468 | 14407 | 431 | 0 |
| b3 | 4689 | 3311 | 9425 | 2270 | 0 |
| b4 | 10102 | 5852 | 5731 | 551 | 0 |
| b5 | 7124 | 4879 | 8263 | 997 | 0 |
| b6 | 3023 | 2678 | 12817 | 544 | 0 |
| b7 | 11436 | 8651 | 4663 | 285 | 0 |

Regions went from 2,534 to 2,590 (1,670 code, 902 data, 18 padding). Observed code went from 25,671 to 26,839 B.
Execution coverage is a separate metric from classification.

## Findings
Details are in analysis/investigation.md, "Round 8". The files are in analysis/tools/round08/: apply.py (with
`--dry`), applied.jsonl, changes.json and new-regions.txt.

- **R7-L041 verified.**
  - covalign reproduces the redirect list: 53 bank-4 Static regions, 1,662 B, 571 opcodes, 38 fully hit, 0 misaligned.
    No opcode is in non-code, and every hit region has prg_banks [8,9,14,15].
  - 38 regions were upgraded whole to Observed, and 15 were split into Observed/Static runs.
  - The result is +1,168 B Observed. The 494 B of split regions that were never fetched stay Static, with their old
    evidence and a split note.
  - Every Observed string carries the caveat "artificial scenario S17 (final-match poker, PkrHandCount and King money
    RAM-patched); not natural reachability".
  - Data 2187 B4_ScriptedCards ($984E-$9852): the evidence names its reader (lda $984E,x at $97FC) and the S17
    observed reads; conf 0.9.
- **R7-L042 verified (metadata only).**
  - 51 Data regions now carry the round-7 commentary role text, a consumer-operand list from a fresh operand scan, and
    then their previous evidence.
  - Confidence went to 0.85 for 40 regions that have a confirmed consumer at the region start.
  - Confidence was not raised for 11 regions:
    - 3268, 2071, 2446, 2192, 2460 and 2642: no direct reader;
    - 2189: only its B4_MoodStrength tail $9A28-$9A2C is read; $9A1B-$9A27 has no reader;
    - 2433: layout unconfirmed;
    - 2457: selected by DMC address byte $F8;
    - 2066: the only operand hits the contained $BD6F;
    - 2755: already 0.85.
  - Kinds and bounds are unchanged. No map labels were added, so commentary labels stay authoritative.

## Leads
Consumed and verified: R7-L041 and R7-L042 (analysis/leads.json updated). No new leads were produced.

## Regions changed (for commentary carry-forward)
- **Provenance only, same id and extent (38 regions, bank 4):** 312, 426-428, 432-437, 441-445, 471, 472, 476, 495,
  497, 505-507, 509, 513-515, 517, 518, 3042, 3051, 3053, 3081, 3087, 3088, 3108, 3114, 3128. Their evidence now
  begins with the S17 Observed note.
- **Split (15 old ids removed; 71 new ids 3374-3444, all bank 4; full list with ranges in new-regions.txt):**

  | Old id | Old range | New ids |
  |---|---|---|
  | 3046 | $8159-$8167 | 3374-3376 |
  | 308 | $816D-$81B2 | 3377-3381 |
  | 309 | $81B8-$82CF | 3382-3398 |
  | 313 | $82EE-$82F5 | 3399-3400 |
  | 314 | $82FB-$8317 | 3401-3402 |
  | 351 | $87A5-$87AA | 3403-3404 |
  | 3082 | $8C3F-$8C8C | 3405-3409 |
  | 3090 | $8D03-$8D10 | 3410-3411 |
  | 3127 | $98FD-$9948 | 3412-3414 |
  | 473 | $995E-$997E | 3415-3417 |
  | 474 | $9984-$99A6 | 3418-3420 |
  | 475 | $99AC-$99FC | 3421-3423 |
  | 496 | $9CA4-$9CFF | 3424-3428 |
  | 508 | $9DF8-$9EA9 | 3429-3441 |
  | 516 | $9F76-$9F90 | 3442-3444 |

  Commentary labels are at CPU addresses, so carry them by address. The new region starts fall on instruction starts
  inside the same routines.
- **Evidence/confidence only, Data:** 2187 plus the 51 R7-L042 regions (list in analysis/redirect/r07-a1/stale-evidence.jsonl).

Commentary updates that follow from R7-L041 (from the r07-a1 handoff):
- **B4_ScriptedCards:** a royal flush in spades for the player. The terminator is dealt as card 0 (the ace) with no
  used check, so a duplicate ace is possible.
- **No longer tentative:** PkrPlayerHandRank $0756, PkrOpponentHandRank $0755 and PkrShowdownWinner 1 = player, and
  the odd GameVar0726 entries are the player's hold flags.
- **PkrHandFlag:** 2 and 3 are never stored by classified code.

## Remaining
- Semantic open items are unchanged:
  - the world-map PlaneTableA/B field meanings;
  - $BB1F-$BB4A and the $BB30 caller;
  - the "MONEY" $9291 reader;
  - DMC records 5+;
  - the read cells of $DFD8;
  - the PkrHandFlag 2/3 selectors.
- 18 Padding regions remain: bank tails, stub gaps and alignment fill.
- Next: commentary round 8 (carry by address, refresh the touched regions and the meanings above), then
  redirect/final editorial acceptance.

## Toolkit
analysis/toolkit-issues.md: no new toolkit issues found. The TK-005 and TK-006 workarounds were used again.
