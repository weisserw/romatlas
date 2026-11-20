# Round 4 redirect handoff (attempt r04-a, 2026-11-20)

Scope: the coordinator question "do the 3 residual bank-1 bytes have any DATA role?", tested with a runtime
read-watch. This was untested before: static searches covered only absolute/indexed operands (bases within 255)
and word pointers, not (zp),y / (zp,x) or computed reads.
- Input checkpoint: pass-004 (maps/current.json sha256 4582999d...; 262157/262160 classified).
- Research only. No changes to the map, state, source, toolkit or ROM.
- Evidence: analysis/redirect/r04-a/
  - readwatch.py, readwatch2.py: scripts
  - readwatch.txt: results
  - ending_fin.png
- Result: **no ready lead.** This file holds the final rejection review for the 3 bytes. **INCOMPLETE: 3 unknown bytes.**

## Residuals (unchanged)
| phys | bank / CPU (context [0,1,14,15]) | byte | preceded by |
|---|---|---|---|
| 0x02DC6 | b1 $ADB6 | $60 | `lda #$64 / jmp LineOut_Add` (FightFish_Update_fish_away) |
| 0x032FA | b1 $B2EA | $60 | `jmp FightFish_PassiveDirection` (FightFish_Passive) |
| 0x03353 | b1 $B343 | $60 | `jmp FightFish_ApplyDirection` (FightFish_DirectionTerms) |

## Method (materially new: runtime consumers in any addressing mode)
- Tool: readwatch.py.
- Run: the configured binary via analysis/tools/rpcd.py, importing the same-build r01-a snapshots from scratch/snap_*.bin.
- Trace settings:
  - kinds: Read + Instruction + Dma
  - space: rom
  - **physical** range [0x02DC6, 0x03354)
- Events were paged every 40 frames and filtered to the 3 offsets. Nothing was dropped.
- The filter is on the physical identity, so it catches:
  - every CPU access mode: (zp),y, (zp,x), abs,x/y and computed bank-relative pointers;
  - any slot alias (bank 1 at $8000, $A000 or $C000);
  - opcode fetches;
  - DMA.
- Dummy-read exclusion was not needed. No event hit the targets at all. r02-a/dummyreads.txt also lists none of the
  3 as a page-cross dummy address.

Coverage: 20 runs, 9,714 traced frames, **0 events at 0x02DC6, 0x032FA and 0x03353**.

| run (snapshot) | milestone | frames | events in range | target hits |
|---|---|---|---|---|
| title | S01 attract | 300 | 0 | 0 |
| harbor | S04/S05 | 250 | 0 | 0 |
| sea (+SELECT menu) | S05/S06 | 412 | 0 | 0 |
| bite (sea, $0496=1) | S08 | 584 | 95,576 | 0 |
| fight_idle / pulse / hold / select / event | S09/S16 | 600/780/768/464/456 | 1.3 M total | 0 |
| land (LineOut=0) | S11 | 686 | 727 | 0 |
| gothim (weigh-in) | S11 | 564 | 0 | 0 |
| dayend / results | S12 | 154/672 | 0 | 0 |
| ending ×2 (results_win to FIN; ending_fin.png) | S14 | 944×2 | 0 | 0 |
| hawaii_info / hawaii_sea | S13 | 436/340 | 0 | 0 |
| hawaii_fight (bite + fight in Hawaii) | S13+S09 | 1,324 | 440,164 | 0 |
| fight_away (ARTIFICIAL $0468 bit6 every 4f) ×2 | fish-away path | 180×2 | 6,853 | 0 |

Exposure of the code just before each byte, from Instruction events (prg_banks [0,1,14,15]):
- `jmp` at $B2E7 (the instruction just before 0x032FA) ran 2,000+ times.
- `jmp` at $B340 (just before 0x03353) ran about 760 times.
- FightFish_Update_fish_away $ADA9 ran 180 times in the artificial rerun, which used range start 0x2DB9.
  - The first-pass `fish_away=0` columns are not measurements: $ADA9 lies outside the range [0x2DC6, ...).
  - This exposure is artificial-state evidence only.

Even with each tail `jmp` running, the following $60 was never fetched or read.

Supplementary static check (new: hi-byte immediates for (zp),y bases):
- Searched for every `ld[axy] #$hh` with hh in {$AC,$AD,$B1,$B2,$B3}, plus the $8000 and $C000 slot equivalents.
- Results:
  - PpuQueue data bytes ($8C/$AC at source lines ~21663/21673).
  - The IRQ hook $F7B2.
  - `lda #$d3` (a signed term).
  - The script pointers $9DB3 and $8C29, both handled by the r01-a script parser, and more than 255 bytes from
    any target.
- None of these forms a zero-page pointer base within 255 of a residual.

## Rejection review (final, per evidence-policy.md "Challenge a terminal decision")
- **Claims tested:**
  - (A) This round: each byte is Data read by some consumer. It would be generic data with a consumer anchor.
  - (B) Round 3 / L-023: each byte is a Static Code RTS ending its routine.
- **Decisive artifacts:**
  - For (A), this round:
    - analysis/redirect/r04-a/readwatch.txt: 0 hits in 9,714 frames across S01 and S04-S16.
    - The hi-byte scan above.
  - From earlier rounds:
    - r03-a: rtsdisp, splitcons, bankrel, twins, refcheck, lonebytes
    - r02-a: absrefs, dummyreads
    - analysis/evidence/round-04/xref.txt
    - The rejection in round-04-disassembly.md.
- **What is ruled out:**
  - Any runtime data consumer in the exercised states, by any addressing mode, slot alias or DMA.
  - Any execution of the bytes in those states.
  - From earlier rounds: every static reference encoding (direct, address-1, ±2, split lo/hi, immediate pair,
    script/hook, bank-relative).
  - Any filler generator.
- **Counterexample scope:** limited to the exercised states. These were not traced in this round:
  - sound test (S10);
  - name entry (S03);
  - password entry (S07);
  - weather/cockpit menus;
  - a naturally produced bite (L-008);
  - the return-to-port menu (S15).

  The 3 bytes sit inside the fight-model code of bank 1. The fight was traced in 7 variants and in both regions.
  So a consumer that only appears in an untraced mode is not excluded, but there is no concrete reason to expect
  one.
- **Independent constraints considered together:**
  - exact 1-byte bounds;
  - no static reference in any encoding;
  - no runtime read or fetch;
  - no filler mechanism in the ROM (Padding is all $FF; no alignment);
  - no `jsr X / rts` twin;
  - no precedent of an accepted unreferenced RTS.
- **Strongest competing interpretations:**
  - Code: a dead RTS epilogue left after a `jsr`->`jmp` tail-call edit.
  - Data: a stray non-code byte with no consumer and no generator.
  - Both predict exactly the observed result: zero reads and zero fetches. The runtime test therefore removes the
    consumer-anchored data reading, but it does not discriminate between the two remaining interpretations.
  - Generic data is not supported either. A single $60 has no repeated pattern, uniform run or visibly
    non-instruction content, and it now has no consumer.
- **Inferred prerequisites:**
  - For Code, a control/stack contract or an independent convention. The census contradicts a convention:
    $B517/$B535 end with the same `jmp ApplyDirection` and have no RTS.
  - For Data, a consumer or a pattern.
  - Neither is available from this ROM.
- **Why no further useful step remains:**
  - Every reference encoding and every consumer mode within reach of this ROM has been tested statically or at
    runtime.
  - The only remaining discriminator is external: the original source, or another revision of the game showing
    these bytes. Downloading ROMs is prohibited.
  - Tracing the remaining non-fight modes would be a low-prospect sweep, and its negative result would not change
    the code-versus-data ambiguity.
- **What would reopen it:**
  - an observed read or fetch of one of these offsets in a new state;
  - a reference or consumer found in later work;
  - a `jsr X / rts` analogue;
  - user-supplied source/revision evidence;
  - a policy change that accepts the "dead epilogue after tail JMP" class.
- **Disposition:** terminal residual. 3 bytes stay Unknown. L-023 stays rejected. No new lead.

## Other unresolved non-classification questions (commentary/uncertainties.md)
| question | concrete next step? |
|---|---|
| Name/password grid ObjX vs ObjY transposition | yes: one frame plus OAM capture on the name-entry screen (cheap, commentary-only) |
| Species name per HookedFishType index | yes: poke the type before landing, then read the weigh-in text (low value) |
| FishAheadHandlers facing 8 mapping; FishState $A0 meaning | partial: compare against observed movers; low value |
| FightFish_AddStaminaFar Ptr2Hi provenance; OffsetPair16/18 contents | partial: RAM watch in the fight; low value |
| FishTypeAdjustSet $05AC / FatigueDrainFlag $05A5 / BitArrayPtr $84 have no writer; FEV_ROD_BROKEN writer | no: static searches are complete |
| Tentative names (GloveHeat, ReelHeat, FightGauge, sound command meanings) | no decisive test; naming only |
| L-008 natural bite; S15 return-to-port menu | deferred: no classification value |

## Leads / scenarios / toolkit
- leads.json:
  - Pass "redirect-r4 (r04-a)" was added. It consumed nothing, produced nothing, and has no ready leads.
  - An attempt was appended to L-023 (data-role runtime test: negative; status stays rejected).
- scenarios.json:
  - The read-watch runs were noted on the milestones they used.
  - S14 was re-confirmed: the FIN screen frame is ending_fin.png.
- Toolkit: no new issues. TK-003 and TK-004 were reproduced (about 3.3 fps traced).
  - Note: Read events carry no PC, so classifying a hit needs paired Instruction events. That is already covered
    by TK-004.
- The rpcd/serve server I started has been stopped. Scratch is unchanged apart from the transient files I removed
  (69 MB, existing snapshots). Free disk is about 3.8 GB.

## Counts
ROM classification: 100.00% (262157 / 262160 bytes classified; 3 bytes unknown). Unchanged from pass-004.
Status: **INCOMPLETE**.
