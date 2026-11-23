# Round 2 redirect handoff (attempt r002-a, input checkpoint pass-002)

**NEXT=disassembly** to review L009 (with a consistency check on the bank 6 stubs) and L010. After that, commentary handles L010/L011.
This round was research only: the ROM, map, state, source and binary are unchanged. Attempt record: analysis/redirect/r002-a/README.md.
Classification is 262155/262160 bytes. 5 bytes are still unknown (exact ledger: analysis/redirect/r002-a/ledger.json):
- bank 13 $1BA6D-$1BA6F
- bank 15 $1FB79
- bank 15 $1FDD0

## Avenues and findings
1. **L009, the 5 unknown bytes.** I reopened this with new combined evidence.
   - **Consumer search** (tools/consumers.py): no absolute, indexed or jmp-indirect operand reaches any of the three sites. Indexed bases were checked down to 255 bytes below each site. There are no immediate lo/hi pairs and no raw words for the target or target-1.
   - **Padding:** the next routines start at $BA60, $FB6A and $FDC1. These are not aligned addresses, so the bytes are not alignment padding. $60 and $4C are not fill values.
   - **Bank 13 $BA5D:** the three bytes are byte-identical to the `jmp DebrisSpawner` four bytes earlier in the same routine.
   - **Consistency conflict:** tools/rtsrts.py finds 5 `rts rts` pairs. Two of them, bank 6 $A175 and $A18B, are lone unreferenced RTS bytes that pass-002 accepted as Static Code (region 1853). Their stated "bank-3 parallel" does not exist. The bank-15 bytes have the same evidence status but were rejected.
   - **Runtime:** none of the 5 bytes was executed in any r002-a route.
   - **Lead split:**
     - L009a (bank 13): stronger, because of the exact duplicate instruction.
     - L009b (bank 15): judge it by the same criterion as the bank-6 stubs.
2. **Bank 8 `11 00 00 01` (L010).** Bytes $B20E-$B212 = `00 11 00 00 01` form an empty 1x1 sprite definition.
   - Ten TransformFrames records point to it, and SetTransformFrame passes them to SetSpriteDef.
   - Byte $B20E is also the shared empty spawn list for levels 3-4 (MapSpawnIndex value $72).
   - The region is already Data, so the classification is sound. Only the label, role and extent description are wrong.
3. **Review of accepted heuristic classifications.** No decoded branch, jsr or jmp target lands in Data or Text (tools/xcheck.py). All earlier observed roots (20656 + 8772) and all 4499 offsets executed in this attempt are Code instruction starts. The only issue found is the bank-6 stub inconsistency in item 1.
4. **L008 routines with no caller:** none were executed in the boss, ending or game-over routes, and no new reference was found. Reachability stays unknown.
5. **Gameplay** (analysis/scenarios.json):
   - **RAM contract:**
     - Energy $0563: $C6 is full; a value >= $DC triggers death.
     - RingLevel $0713: remaining lives.
     - EndingFlag $0715: $FF means game over.
     - EventPhase $F5 and GoalHitCount $0709.
     - Boss zone: bank 7 $B0CA creates task $A015 in CpStageBank, which is CreateInitialObjects(13).
   - **Visited:**
     - **Real game over** (artificial patch; the sequence after it ran on its own): Gaia "I AM VERY SAD... THE PLANET IS DYING", then the FINAL SCORE / CONTINUE / END menu. CONTINUE restarts the stage; END returns to the title.
     - **Level-3 boss arena:** the boss escaping off the right edge restarts the arena and costs a life.
     - **Level-4 boss tasks:** patching GoalHitCount to 5 gives the victory pose, then the "EARTH IS SAVED" ending.
     - **Level-1 CP stage**, plus its goal objects.
   - About 900 traced frames added 585 executed opcodes in Static code (banks 7, 12, 13, 14, 15), with zero contradictions.
   - The commentary misreadings found this way are packaged as L011.

## Ready leads (analysis/leads.json)
| ID | Phase | Scope | Expected |
|---|---|---|---|
| L009 (a/b) | disassembly | 5 bytes as Static dead code, decided by one criterion. If L009b is rejected, return bank 6 $0C185 and $0C19B to Unknown. | 5 B (or a consistent residual) |
| L010 | disassembly/commentary | Optional split of region 1500 at $1121E; label EmptySpriteDef8 | 0 B, role precision |
| L011 | commentary | See the list below | ~10 symbols and notes |

L011 corrections:
- Render the $B0CA word as numeric `$A015` in CpStageBank.
- Rename CreateInitialObjects to a boss-zone name.
- Correct the game-over and ending names (StageEndDispatch, StartGameOverSequence, GaiaMessageTask, GameOverMenuTask).
- Fix the RingLevel and EndingFlag notes.
- Fix the Level3BossTask loss reading.

## Deferred
- **Level-0 goal and an actually played boss defeat:** the mechanism is the same as the visited stages; the only gain would be gameplay coverage. The recipe is in scenarios.json.
- **L008 reachability:** there is no reference and no execution; further search has a low prospect.
- **Bank 12 $A256 / bank 7 $B77C context (L005):** no byte depends on them.

## Rejection review
This is not terminal, so the full review is not required. For L009, the decisive artifacts are tools/consumers.py (negative) and tools/rtsrts.py (consistency). What they rule out: a data consumer reachable from decoded code, and alignment or fill padding. What they do not rule out: an unseen computed (zp),y pointer. The strongest competing interpretation is a one-byte or three-byte data remnant with no reader. No evidence supports that reading.

## Other notes
- **Possible deletion of other tasks' scratch files:** during cleanup I ran `rm -rf /tmp/*.json /tmp/rshots/*.png`. Besides my own scratch, this may have deleted top-level /tmp/*.json files that belonged to other tasks. No workspace evidence was affected.
- **Server:** stopped.
- **Disk:** r002-a uses 5.0 MB (one cited snapshot); about 3.1 GB free.
- **Toolkit:** no new toolkit issues.
