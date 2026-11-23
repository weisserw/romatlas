# Round 1 redirect handoff (attempt r001-a, input checkpoint pass-001)

**NEXT=disassembly** with ready leads L001, L002, L003, L004, L005, L007 and low-priority L008.
Research only: ROM, map, state, source and binary are unchanged. Attempt record: analysis/redirect/r001-a/README.md.
Classification is still 92.64% (19288 unknown). The proposed batch would cover about 18.97 KiB of that; it has not been accepted yet.

## Avenues investigated and findings
1. **Bank 8/13 Captain Planet object code (L001).** I decoded MapSpawnTable ($B19C) and LevelInitTasks ($AA20/$AA23) in bank 8, and
   MapSpawnTable13 ($A6C6) and LevelInitTasks13 ($A3C0/$A3C5) in bank 13. They give 34 + 23 task entry words (roots_spawn.json).
   Entry contract: after CreateTaskAt the spawner writes the constant 8 or 13 to TaskBank ($052B), and the dispatcher maps it into R7.
   Traversal: +4668 code bytes in bank 8 and +5098 in bank 13. No instruction overlaps, BRK or unofficial opcodes.
   Runtime corroboration (artificial password routes):
   - 12 live task resume PCs fall in unknown bytes, and all of them are traversal instruction starts.
   - 165 observed bank-8 fetches fall in unknown bytes, at traversal starts.
   - Across all 8772 roots observed in this attempt, none contradicts the traversal.
2. **JMP-table dispatchers (L002).**
   - Bank 3 SpecialObjectHandlers: 8 words. The bound is the SpecialSpawnTable base $A3CD.
   - Bank 6 EventObjectHandlers: 26 words ($A2FE-$A331). Entries from 26 on are signed pairs, which a separate reader accesses through base $A373.
   - Result: +1375 code bytes in bank 3 and +709 in bank 6. Bank 12 $A8F3 needs nothing; its targets are already classified.
3. **Fixed-bank tasks and data (L003).**
   - $F1E6 is the post-Captain-Planet-stage "WELL DONE / PASSWORD FOR THE NEXT LEVEL IS" portrait screen, not a game-over screen.
   - $F3D9 holds the credits sprites. I observed both executing after artificial task starts.
   - Consumer-proven data: $F218, $F494, $F4B2-$F6FA (whole credits-sprite structure, exact), $C639 (126 words of i*$22), $C88B, $D252.
4. **Heuristic Data review (L004).** Two localized counterexamples:
   - Region 1404 contains code at bank 3 $AECC-$AEE0.
   - Region 1437 ends with SetTaskBank8 at bank 8 $B213-$B21B.
   No other overlap between the 48651-byte traversal and existing Data or Text.
5. **Same-bank task inference (L005).** In 49 of 52 creation sites the caller's own bank constant is written to TaskBank.
   - Bank 7 $B0CA really uses CpStageBank. That record is wrong, but it adds no bytes.
   - Bank 12 $A256 and bank 7 $B77C still need a manual look.
6. **Residual complement.** analysis/redirect/r001-a/ledger.json holds 100 exact rows: data 6634 B, text 10 B, code-candidate 279 B, ambiguous 35 B.
   - Sprite-definition chains (format [attr][rows<<4|cols][dx][dy]+tiles) close exactly at the gap ends:
     - bank 8 $BA88-$BFC2 (123 defs)
     - bank 7 $BD6D-$BFEF (55)
     - bank 13 $BB78-$BFF0 (91), followed by $00 fill
     - bank 12 $BF19 (21)
     - bank 15 $E9AB (21; read through the BriefingTextTable record words) and $E814
     - bank 6 $B985 (50 defs, all referenced by EventFrames)
   - The 10 single bank-12 bytes are text: $00 terminators and $5E apostrophes.

## Ready leads (recipes are in analysis/leads.json)
| ID | Scope | Expected |
|---|---|---|
| L001 | banks 8/13 task code from roots_spawn.json | ~9.8 KiB Code |
| L002 | banks 3/6 handler roots + table bounds | ~2.1 KiB Code |
| L003 | $F1E6/$F3D9 code + bank 14/15 data rows | 237 B Code, ~1.1 KiB Data |
| L004 | split Data regions 1404 and 1437 | 30 B Data->Code |
| L005 | accept 49 same-bank sites; note the $B0CA CpStageBank exception | validation |
| L007 | ledger rows marked data/text | ~6.6 KiB Data/Text |
| L008 | 12 fragments + 8 JMP-slot/single-byte rows | up to 314 B (review) |

Common recipe: `cd analysis/redirect/r001-a && EXTRA=roots_all.json TRAV_OUT=/tmp/t.json python3 -I tools/trav2.py && python3 -I tools/cmp.py /tmp/t.json`.
It must show only the two L004 conflicts. Then annotate. Observed provenance comes from runs/obs_roots_r001a.json.
Re-run `tools/residual.py` afterwards; only rows marked code-candidate or ambiguous should remain.

## Gameplay (analysis/scenarios.json)
- Visited:
  - Level 3 Captain Planet stage (snapshot runs/l3cp_start.json)
  - Level 5 Captain Planet stage (snapshot runs/l5cp_start.json)
  - the level-clear password screen, reached by artificially starting StageClearTask $E2E8 with ContinueFlag set
  - the credits scene, reached by artificially starting CreditsTask $E39A
- Deferred:
  - **Bosses:** level 5 shows a large structure, but I did not engage it. The boss-phase contract (EventPhase=3 in SpawnFromMapCell13) is not yet understood.
  - **Real game over:** the lives/health RAM contract has not been identified.
  - **Level 2 Captain Planet stage:** it uses the same bank-8 object family as levels 1 and 3, so a visit adds little.
  - Next practical step for all three: find the health variable in CpPlayerTask (bank 7) and patch it to reach game over.

## Other avenues not pursued
- Who calls StepObjectAnimation13 with index >= 13, which would prove the $A107 table base: the call chain through CallCpStageBank_* is mislabelled, so this is low value. It is generic data either way.
- Natural callers of the duplicate routines in L008: the word and immediate-pair searches turned up nothing, so the prospect is low.
- Toolkit: no new issues beyond TI-001..TI-004. I worked around TI-004 (slow tracing) by sampling the task table without tracing.

Disk: the attempt directory is about 11 MB (two 5 MB snapshots, both cited). Scratch was in /tmp. The JSON-RPC server was stopped at handoff.
