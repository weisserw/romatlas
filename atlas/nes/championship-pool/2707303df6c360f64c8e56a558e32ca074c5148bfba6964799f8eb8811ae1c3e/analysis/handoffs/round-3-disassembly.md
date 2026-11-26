# Round 3 - disassembly handoff

Status: complete. This was a metadata-only pass. Every byte was already classified, so there were no byte gains to
take. Input checkpoint: pass-002-final. Output checkpoint: **pass-003**.

## Scope
- I restored analysis/state.json with map.load. All existing classifications are kept.
- I consumed ready lead **L-009** (map metadata refresh) using analysis/tools/annotate_r3.py, in a dry run followed by
  `--apply`. Before it edits anything, the script re-checks each fact from the ROM and the saved traces:
  - **Region 989** (bank 7 $F9A9-$FA3E, physical $1F9B9, DrawBigNumber). New evidence:
    - Five static `jsr $F9A9` callers in bank 3, at $96E8, $970B, $972E, $974C and $9772. They are the only
      jsr/jmp references to it in the ROM.
    - 74 of its 77 instruction starts are observed in the **artificial-state** S13 trace (GameCmd $D2=$19 patch).
      The unobserved starts are $F9C4, $F9C7 and $F9C9.
    - The evidence states that this shows execution under the patch, not natural reachability.
    - Confidence: 0.7 -> 0.9. Provenance stays Static because observation is partial.
  - **Regions 974 and 1112** (bank 5 $8A3D-$8A47 and $8A48, physical $14A4D and $14A58).
    - They are the statically reached fall-through of AimAtCalledBall ($8A3A, region 436, called from bank 7 code)
      after a returning `jsr SelectBank5_Temp`.
    - Confidence: 0.7 -> 0.9. The stale "no caller" text was replaced.
  - **Region 1122** ($DE1C pha, physical $1DE2C).
    - Added the stack-contract evidence: VramBufFlushNow pushes 2 bytes and pulls 3, and both branch-in users pha
      first. A direct entry therefore needs exactly this one pha.
    - Confidence: 0.5 -> 0.65. Reachability is still not claimed.
  - **S14 wording, 10 Observed regions** (43, 173, 637, 643, 645, 655, 657, 692, 938, 959).
    - "then natural interview/menus" became "then unpatched execution of the interview/menus after that artificial
      shortcut; not natural reachability".
  - **Opponent-message wording, 6 Observed regions** (1060, 1062, 1123, 1125, 1127, 1128).
    - "natural reachability is likely" is now explicitly labelled as a judgement, not evidence.

## Region boundaries and kinds
**None changed.** I diffed map.get output from before and after the edits. Exactly 20 regions changed, and only
their evidence and confidence fields. Offset, length, kind, provenance, label, comment and views are identical. No
regions were added or removed (1129 before and after).

## Checkpoint and verification
- map.validate: ok.
- map.save analysis/state.json: done.
- map.export maps/current.json: done.
- `python3 run.py checkpoint pass-003`: ok, **byte_identical**. Map sha256 begins 2d5ac8c3; the full result is in
  analysis/pass-003-verification.json.
- input/game.nes is unchanged (sha256 2707303d...).

ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown).

| Bank | Code | Data | Padding | Unknown |
|---|---|---|---|---|
| header | 0 | 16 | 0 | 0 |
| PRG 0 | 0 | 16165 | 219 | 0 |
| PRG 1 | 0 | 16384 | 0 | 0 |
| PRG 2 | 3495 | 12755 | 134 | 0 |
| PRG 3 | 15023 | 1361 | 0 | 0 |
| PRG 4 | 0 | 16369 | 15 | 0 |
| PRG 5 | 13298 | 3045 | 41 | 0 |
| PRG 6 | 2309 | 13705 | 370 | 0 |
| PRG 7 | 11619 | 4676 | 89 | 0 |

Low-confidence code (confidence < 0.75) went from 878 bytes in 56 regions to 716 bytes in 53 regions.

## Leads
- **L-009: verified.** The outcome and history are recorded in analysis/leads.json. Do not retry it.
- L-010 (commentary findings) was not touched. It belongs to commentary or final acceptance.
- No new leads. No gameplay was run, because it could not yield any structural gain.

## Toolkit
No toolkit issues were found in this pass. analysis/toolkit-issues.md is unchanged; TK-001..TK-003 still stand.

## Processes and disk
- I started an rpcd daemon (PID 2160951) and its serve child (PID 2160953). I stopped both by their own PIDs.
- I removed the scratch directory /tmp/ra-2707-r3d.
- I did not modify commentary/.
