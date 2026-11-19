# Round 1 — redirect handoff (attempt r1-a2)

**Recommendation: NEXT CYCLE TO DISASSEMBLY.** Taken together, the r1-a1 and r1-a2 proposals cover
**4,748 of the 4,750** unknown bytes, plus the 142-byte data->code correction for R-06. R-06 has now
been observed executing. Only 2 bytes remain unknown (file 0x1C5AE-0x1C5AF), with a stated reason below.

## Scope and checkpoint
- Input checkpoint **pass-001** (analysis/state.json == maps/current.json; analysis/pass-001-verification.json
  byte_identical). This phase made no map, state or source edits. Classification is unchanged at 98.19%.
- Research directory: `analysis/redirect/r1-a2/`. It holds tools/, proposals.json, residual.txt, l014.json,
  msprite_bank5_marked.json, k42kill.log, k42pos.log, k42near.log and observed/contexts.json (this attempt's traces, tags k42kill/k42near/k42pos).
  The r1-a1 records are unchanged.
- **One batch for disassembly:** `analysis/redirect/r1-a1/proposals.json` (3,372 B + R-06 correction) plus
  `analysis/redirect/r1-a2/proposals.json` (1,376 B). r1-a2 groups were computed on the residual after r1-a1.
  combine2.py checks every byte: it must be currently unknown, outside r1-a1 and not in another group. It found 0 conflicts.
- Binary /root/romatlas/target/release/romatlas via `run.py serve` (relay analysis/tools/rpcd.py); the server is shut down.
  Disk: about 4.5 GB free; scratch is 0.8 MB.

## Findings (r1-a2 proposal groups; all Data, Static provenance)
| Group | Lead | Bytes | Exact basis |
|---|---|---|---|
| D-AREA6/7/8 | L-014 | 730/236/106 | Area banks (mapped at $A000) start with per-map 8-byte metatile tables (Dat_14_C404) and 120-byte screen cell tables (Dat_14_C422; screen s is at s*$78 via AttributeShift/MaskTable). Used screens come from rooms flooded from the area-start record through the $C02A wall map. **15 of 18 table anchors tile exactly** against the next table base or classified code (bank 6 block ends at code $BBB0, bank 8 at code $B357). The rest leave 3-7 spare records or one aliased screen. Every cell value is below its metatile capacity. No observed fetches fall in the gaps. Tools: l014.py, l014.json |
| D-HITBOX7 | L-017 | 51 | ObjHitboxIndex $9C16 has 195 B (the normal metasprite count). BossHitboxIndex $9CD9 has 178 B. ObjHitboxes $9D8B has 49 x 4 B (max index used is 48). Together they run from code ending at $9C15 to code at $9E4F with no gap |
| D-SPAWN7 | L-017 | 5 | Spawn-list walk (spawnwalk.py) covers 0x0ED83-0x0F948 and ends at code 0x0F949. The gap lies inside an 11-byte entry |
| D-B5SCR8 | L-017 | 11 | Boss-type-5 script: the 57 frame counts sum to 69, so there are four 69-byte columns; Dat_08_9942/9951 then end at code $9960 |
| D-DEATH8 | L-017 | 7 | DeathAnimChr,x for x=0..7 ($9F36-$9F3D) |
| D-B8DUP | L-017 | 85 | Bank 8 $9F3E-$9FFF is byte-identical to bank 4 $9F3E-$9FFF. There is no bank-8 reference into it, and it starts right after DeathAnimChr. Same disposition as r1-a1 D-B4DUP |
| D-CSPAL2 | L-016 | 22 | CsPaletteSets: 21 pointers -> 21 consecutive 8-byte sets $8E34-$8EDB, ending at CsColourTriples. Max index 75 = 76 triples, which end at $8FC0 |
| D-CSANIM2 | L-016 | 61 | CsAnimScripts: 8 pointers -> 8 typewriter scripts (PPU address, bytes, $FF), with no gap from $8FD0 to $966A; the cutscene metasprite table starts at $966B (csanim.py) |
| D-BSPR5 | L-016 | 62 | Boss metasprite lists (msprite.py 195/189, 57 B), plus the 189-word boss tile pointer table $9A7E-$9BF7 (5 B) |

Note: the r1-a1 msprite walk already marked the 57 bank-5 bytes, but its combine output omitted them. r1-a2 adds them.

## Gameplay / R-06 (artificial; scenarios.json S5 updated)
- Death reload into area idx 17 ($88=$22, $B7=0, $BE=3; 240 frames, then holding Right) brings in kind 42 (HP 160) and kind 44.
  CHR graphics are garbled, as with other S2 reloads.
- Forced defeat (ObjState|=$40, ObjVarE=0, ObjTimer=0) runs ObjKind42 -> `JMP $BCB4`. This fetched **67
  instruction starts in bank 13 $BCB4-$BD54**, context [12,13,14,15], on the static boundaries.
  **L-007 is confirmed in an artificial scenario.** Merge these fetches as observed for the fetched instructions.
- The clear branch needs |boss X - Batman X| < $20 (ObjDistanceToBatman $BC6D). Walking was blocked at X=423,
  so Batman X was patched to boss X+16 ($D3:$D4). The routine then set BossDefeatFlag $CE=$80 and removed kind 42.
  The area-clear flag $CF was not seen within 240 frames, as with idx 16.
- Every other new fetch is already Code/Static, so no conflict with any data proposal.

## Unresolved / deferred (with reasons)
- **0x1C5AE-0x1C5AF (bank 14 $C59E, 2 B), left unknown.** These bytes follow `jmp (Ptr0)` and come just before
  HideUnusedSprites. They decode as `lda $F5`. No word or branch reference exists, and the byte sequence appears nowhere else.
  An alternate entry (A = joypad) and 2 bytes of filler are both plausible; no remaining constraint tells them apart.
- Post-defeat area clear for idx 6, 16 and 17 was not observed (S5 partial). After BossDefeatFlag, the clear probably
  depends on another position or timer condition. This would add gameplay coverage only, with no byte gain, and there is
  no concrete lead, so it is deferred.
- The area-map screen model in banks 9/11 over-includes aliased ids (map 4 id 22 = map 6 screen 0; map 7 id 25 =
  map 8 screen 11; map 5 id 13 crosses the bank end). These banks are already fully classified, so the only effect is
  that the per-screen model is approximate. The block-level claim for banks 6/7/8 does not depend on it.

## Rejection review (compact)
Only the 2-byte $C59E remains. **Claim:** code (an alternate entry) or data. **Decisive evidence:** it sits after an
unconditional indirect jump; findw.py and a raw byte search find no reference; it is not a copy of any other bank;
there is no observed fetch. **What this rules out:** reachability via a known caller only; code is not disproved.
**Strongest competitor:** a stale entry stub. **Why the inference is insufficient:** 2 bytes give no record, terminator or
consumer constraint. The disassembly reviewer should keep it unknown unless new evidence of a caller appears.

## Leads
leads.json pass entry r1-a2: added L-017 (ready); updated L-007 (observed evidence), L-014 (ready), L-016 (ready) and L-003.
Ready for disassembly: L-007..L-017. L-005 is still listed as ready from pass-001.

## Toolkit
No toolkit issues found in this attempt (analysis/toolkit-issues.md unchanged).
