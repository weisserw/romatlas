# Round 4 / redirect handoff (attempt r04-a1)

Status: **complete, proposal ready** (not terminal). The proposal is role, boundary and provenance only. It gains 0
classification bytes.

Checkpoint: pass-004 (maps/pass-004.json sha256 eb7719e3...9519 == analysis/state.json == maps/current.json).
- Classification: 131,088 / 131,088, with 0 unknown. This attempt did not change it.
- Not modified: map, ROM, source or toolkit.
- Research is in analysis/redirect/r04-a1/: findings.json, tiling.txt, immpair.txt, ctlcheck.txt,
  covcheck_gameover.txt, static_observed.json, frames/gameover.png, and tools/ (immpair.py, strtile.py, b5intro.py,
  ctlcheck.py, covcheck.py, gameplay/go3.py).
- Runtime evidence is in traces/redirect-r04/.
- The scratch dir /root/rascratch-r4r has been removed and my server has stopped.

## Scope
The coordinator's questions:
1. The bank-3 $92B0/$92D0 string-table cut.
2. Code-versus-data conflicts in the 0.75 generic windows and the low-confidence fragments.
3. Inconsistencies between the map and commentary.
4. Later-game milestones.

## Findings
1. **Bank 3 $9270-$9974 is one structured block** (R4-L030). The commentary noted only part of it. Its parts:
   - Opponent name pointers and names.
   - B3_PanelLabelStrings ($9292-$92ED).
   - Bank3_Data_92EE: 4 x 18-byte per-StakeTier records, indexed StakeTier*$12 with an 18-byte copy at $80FD. Each
     record holds two 32-bit amounts plus 10 bytes.
   - **B3_MessagePtrs $9336-$9465**: 4 tiers x 38 words. The base comes from the immediates `adc #$36` ($816E) and
     `lda #$93` ($8173), with stride $26*2. B3_PrintMessage reads it through zpMessageTable.
   - **56 $FF-terminated messages $9466-$9974**. They tile exactly up to the code at $9975.

   Answer to the coordinator's question: the table's real bounds are $9292-$92ED. Its kind is correct, but about 30
   phase-1 windows cut every sub-structure, and the commentary states falsely that they have "no code reference".
2. **Same defect in bank 2 and bank 5** (found by a broader immediate-pair scan that r02 immscan2 missed):
   - R4-L031, bank 2 $B323-$B6EE: two attract-text pointer lists. $B58F has 35 words and $B6D1 has 15. B2_AttractSetup
     loads them into zpLocalC/D, and B2_AttractUpdate reads them through Api_RunTextMenu. Both lists tile their $FF
     strings exactly.
   - R4-L032, bank 5 $BAEA-$BC21: B5_OpponentIntro uses a 6x6 frame block and two-level tables of VRAM text blocks.
     The table at $BB10 has 3 entries. The table at $BB5A has 3 entries, which point to sub-lists of 2, 3 and 4 entries;
     these counts equal the zpLocal4 row counts. The [w,h,tiles] blocks tile exactly to the padding at $BC22.
3. **No code-versus-data conflict** (ctlcheck.py):
   - 4,293 bank-aware jsr/jmp/branch edges land 0 times in data or mid-instruction. The 2 hits are fixed-bank
     `jsr $8012/$8015` trampolines into the switched bank, which is a tool limitation.
   - Every immediate pair that targets a generic window is a data pointer or a numeric constant.
   - The 8 low-confidence fragments decode cleanly, and no new reference to them exists.
   - Small template-wording issue: the evidence for region 2765 ($90E5) says it "joins established code". In fact it
     ends in its own rts before the dword table at $9108.
4. **Game over visited** (S16, artificial; R4-L033).
   - Recipe: start from the final-match roulette snapshot, write winning number 5 and PlayerMoney 0, then press Start.
   - Result: "YOU LOST EVERYTHING", then GameOutcome = 2, then MainLoop_AfterGame $C1A3 runs, then the GAME OVER
     screen, then the title.
   - Coverage: 5,289 opcodes, 0 in non-code, 0 in low-confidence fragments. 42 Static regions (656 opcodes) were
     observed for the first time, including the bank-3 post-game code $9975-$9C0C.
5. **Symbol notes** (editorial):
   - StakeTier's comment "(0..2)" describes the stored table value. The in-game value is 0..3, and the bank-3 tables
     have 4 entries.
   - GameOutcome 2 = player lost everything. This is now confirmed.

## Ready leads (analysis/leads.json; 0 B gain each)
| ID | Target | Recipe |
| R4-L030 | b3 $9270-$9974 (phys 53888-55684) | `strtile.py 3 9336 152`; realign Data at $9278/$9292/$92EE/$9336/$9466; names B3_TierParams, B3_MessagePtrs, B3_Messages; `#<`/`#>` at $816E/$8173; drop false "no reference" notes |
| R4-L031 | b2 $B323-$B6EE (phys 45875-46846) | `strtile.py 2 B58F 35` / `B6D1 15`; four Data regions (split 2054 at $B6EF); symbolic immediates at $B192-$B1BA |
| R4-L032 | b5 $BAEA-$BC21 (phys 97018-97329) | `b5intro.py`; regions at $BB10/$BB16/$BB5A/$BB60/$BB72; symbolic immediates at $B96F/$B827 |
| R4-L033 | 42 Static code regions (static_observed.json) | covcheck.py on the gunzipped cov_gameover; provenance Observed (artificial S16); MainLoop_AfterGame/GameOutcome comments |

## Rejected / deferred
- Code conflicts in generic windows: none found by control edges, immediate pairs or coverage (rounds 2 and 4).
  Diminishing prospects.
- Low-confidence fragments: there is no reference, and none executed in any run, now including game over. The
  missing fact is a caller. Keep them as they are.
- Other immpair.txt hits (e.g. b4 $86A0, $FFxx, $E0xx) are constants. The hits that point to targets the commentary
  already names were not re-examined.
- Full credits scroll (S15): it was already visited artificially (S12). It is not structurally needed.

## Weakest classifications (compact review)
- 0.75 windows: I checked them for code conflicts and found none. Several now have known roles (L030-L032).
- 0.5-0.6 fragments: these are structure-only claims, with reachability unresolved. The evidence has not changed.
- $BB30 (0.5) was accepted in round 4, and nothing new contradicts it.

## Recommendation
Run a short **disassembly round 5** to apply R4-L030..L033. These are boundary, role and provenance changes only, but
they correct false commentary claims. Then run commentary and final editorial acceptance. A further redirect is not
needed unless round 5 finds a problem.
