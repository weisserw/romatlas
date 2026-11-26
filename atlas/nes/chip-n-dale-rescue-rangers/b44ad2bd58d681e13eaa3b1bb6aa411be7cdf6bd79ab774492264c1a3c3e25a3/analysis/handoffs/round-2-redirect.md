# Round 2 redirect - handoff (attempt r2-a1)

## Checkpoint
Input is pass-002 (maps/current.json == analysis/state.json, 262159/262160 classified). The only unknown
is fixed $F62C (phys 0x1F63C, 8 KiB PRG bank 15, byte $60). Redirect changed only research and lead
records. ROM, map, state, source and binary are unchanged. Research is in analysis/redirect/r2-a1/
(notes.md, tools/, rtsjmp-scan.txt, twop-*.json/txt, provenance-upgrade.txt).

## Findings
1. **$F62C is a dead-epilogue RTS (new combined inference).** `PlayerBump_No_Skip: clc ; jmp
   PlayerHitSolidSide ; rts`. The `clc` is dead because PlayerHitSolidSide, entered only through this JMP,
   overwrites C on every path before any read. `clc ; rts` is the routine's "no bump" return, mirroring
   `sec ; rts` (PlayerBump_Yes), and callers test it with `jsr PlayerBump ; bcc`. The JMP was spliced
   between the two instructions, so the RTS is a leftover instruction of the routine body.
   Negative checks rule out a data or dispatch role:
   - The only indexed table whose base could cover $F62C is AimDirectionTable; its index is at most 30
     (bounded).
   - There is no $F62B/$F62C word, no hi/lo split pair, no immediate pair, and no real PHA;RTS
     dispatcher.
   - The address is odd, so there is no alignment filler.
   - Both neighbours are Observed code.
   In-code precedents: all 6 other RTS-after-JMP instances are branch targets, and the 51 untargeted
   post-transfer starts are table-dispatched handlers. So reachability is **not** claimed.
2. **Natural 2-player game (milestone M-2p, previously deferred) visited.** This used title menu inputs
   only, with no RAM edits. PlayerBump_LiftPartner and the partner-bump paths executed. $F62C was not
   fetched. Every instruction of 14 Static code regions (230 B) was fetched, giving an Observed
   provenance upgrade.
3. **$6000 cart RAM.** Only DemoInputRecorder addresses $6000-$67FF. The header has no battery, and
   iNES 1.0 byte 8 = 0, so the board's WRAM is undecidable from the image. Classification is unaffected.

## Ready leads (analysis/leads.json)
| Lead | Action | Gain |
|---|---|---|
| L-015 | map.annotate phys 0x1F63C len 1: Code, Static, conf 0.6. View cpu $F62C, prg_banks [null,null,14,15]. Label PlayerBump_DeadRts. Comment: "unreachable epilogue remnant; reachability not claimed". Verify that it decodes as RTS, then generate/verify byte-identical. Fallback if the reviewer rejects code: 1-B filler Data 0.6 (the $803F precedent). | 1 B, so 100% classification |
| L-016 | Set provenance Static->Observed for the 14 regions in provenance-upgrade.txt (ids 931,1661,1010,1307,1349,1363,1401,1424,1435,1445,1459,1561,1573,1575). Evidence: twop-cov.json (natural 2P route). Optionally split the 6 partial regions. | 230 B provenance, no new bytes |

## Rejection review (remaining residual questions)
- **Callers for L-011/L-012 routines.** Premise: a caller should exist. Counterexample scope: the r1 raw
  scans and this pass's RTS-trick, split-table and immediate checks are all negative. Missing
  prerequisite: any indirect route, and no new evidence for one exists. No corrected inference remains.
  They stay structure-only.
- **Semantic unknowns and the 28 base-offset operands.** All bytes involved are classified. The
  remaining work is naming and extents, which are commentary or editorial items. The only possible
  runtime test (bonus-stage pick-up identification) has semantic value only.
- **Board WRAM.** The answer is not encoded in the image, so no test can resolve it.

## Recommendation
Run one more short disassembly pass (L-015, L-016). Then do a commentary touch-up: the $F62C comment,
and Observed markers in the regions L-016 upgrades. After that, run the final editorial audit and
verification. Once L-015 is applied, no further redirect avenues remain.
