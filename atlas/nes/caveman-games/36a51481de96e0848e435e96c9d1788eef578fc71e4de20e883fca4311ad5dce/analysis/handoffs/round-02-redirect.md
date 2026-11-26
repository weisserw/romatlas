# Round 2 — redirect handoff (attempt r02-a1)

**Recommendation: (a)** Run one more disassembly → commentary → redirect round on the small ready batch below.
The batch contains one real code/data misclassification (L-013), one table/text boundary error (L-014), a
factual commentary error (L-015) and provenance upgrades from new natural execution (L-016). L-017 and L-018
are optional fidelity items. None of them changes the 100% classification count. After this batch, no
promising avenue remains open, and final editorial acceptance should follow.

## Scope and checkpoint
- Input: pass-002 (maps/current.json == maps/pass-002.json; analysis/state.json). 100.00% classified:
  262160/262160 bytes, 0 Unknown. The unknown complement is empty, so no ledger groups are needed.
- Not modified: the map, state, source and ROM (input/game.nes sha256 36a51481...). No toolkit change. Nothing
  published or archived.
- Research record: analysis/redirect/r02-a1/notes.md. Scripts, outputs, coverage and snapshots are in the
  same directory.

## Findings
1. **Whole-map static audit** (audit.py → audit.txt). I decoded every Code region in its bank context
   (18316 instructions) and checked the map. Results:
   - JSR/JMP targets that miss an instruction start: 0.
   - Branch targets that miss an instruction start: 0.
   - Undefined opcodes in Code: 0. Region overruns: 0.
   - Observed fetches outside Code: 0 out of 14188.
   - The 6 Code runs that end in a conditional branch were checked. Each branch is always taken.

   The established Code graph is consistent. The remaining errors are local.
2. **L-013 (real misclassification).** Bank 3 $860E-$8614 is classified as Data, as the tail of
   DR_ActionRiderX. It is code: `LDX #1 / JSR DR_DrawRacers / LDX #0`, which then falls into DR_DrawRacers.
   - Extent proof: DR_ActionBase plus DR_ActionTicks split the indices 0..82 exactly (base[i]+ticks[i] =
     base[i+1], and 51+32 = 83).
   - The parallel tables DR_ActionFrame and DR_ActionRiderY already have exactly 83 entries.
   - So DR_ActionRiderX ends at $860D.
3. **L-014 (boundary).** The Caves of Fame consumer reads 6 offsets and 20-byte titles (LDY #$14, max offset
   $50, FameEventIdx 0-3 and 5). It fixes FameTitleOffsets = $8547-$854C and FameTitles = $854D-$85B0.
   - The map puts the offset byte $854C inside a Text region.
   - It puts $85B1, the opcode of a self-jump `JMP $85B1`, inside Text, while that instruction's operand sits
     in a separate Data region.
4. **L-015 (commentary fact).** The commentary says "only bank 1 executes CLI". That is wrong:
   - Banks 1, 4 and 5 execute CLI. Their IRQ entries are bare RTIs, and IRQs were observed at bank 4 $804E
     and bank 5 $803D.
   - Banks 0, 2, 3 and 6 route their IRQ to IrqHandler ($C000) and never clear I.
   - The conclusion (IrqHandler is static-only) stands. Only the stated reason needs fixing.
5. **Multi-player mode (S15 → visited; L-004 partly resolved).** This used natural menu input only, with no
   RAM writes. With 3 players, Practice Clubbing executed CL_Bracket3. With 6 players, it executed
   CL_Bracket6 and CL_PairFromWinners.
   - Players pick on alternating keypads 1/2, and each pick needs 2 A presses (select + biography).
   - The runs added 144 new instruction starts in 21 Static Code regions, with 0 outside Code.
6. **Dead or self-modifying fragments** (L-017, L-018). These follow the L-011 precedent and are optional.

## Ready leads (analysis/leads.json)
| ID | Range / context | Action | Effect |
|---|---|---|---|
| L-013 (medium) | file 0xC61E-0xC624, bank 3 $860E, ctx [6,7,14,15] (region 1963, exactly these 7 B) | Data→Code, Static conf 0.8, unreferenced, falls into DR_DrawRacers. Commentary: DR_DrawBothRacers; DR_ActionRiderX = 83 entries. Re-check: audit.py (F list), bytes at file 52651/52663. | 7 B fidelity/boundary |
| L-014 (low) | file 0x557-0x5C3, bank 0 $8547-$85B3, ctx [0,1,14,15] (regions 34-42) | Data 0x557-0x55C (FameTitleOffsets); Text 0x55D-0x5C0 (FameTitles, 5×20); 0x5C1-0x5C3 as one item. Make it Data, following the self-jump precedent at $89ED/$902F/$8987, or Code. | boundary/role fix, about 108 B |
| L-015 (low) | commentary only: bank7.asm IrqHandler header, uncertainties.md | Correct the CLI/IRQ-vector reason. | accuracy |
| L-016 (low) | Static Code regions hit in cov_new_r02.json (21 regions, incl. CL_Bracket3 0x154A1, CL_Bracket6 0x1555A, CL_PairFromWinners 0x15598) | Upgrade provenance to Observed, splitting where only partly fetched (as in L-012). Record L-004 reachability for CL_Bracket3/6. | provenance only |
| L-017 (low, optional) | bank 0 file 0x12A, 0x18A, 0x1D3 (8 B each) | Dead `LDA #1/STA FrameTimer/STA $0626` fragments: Data→Static Code 0.7, as in L-011, or keep Data. | 24 B fidelity |
| L-018 (low, optional) | bank 2 file 0x8373-0x8382 ($8363) | Self-modifying PPUMASK emphasis fragment (STA hits its own operand). It cannot work from ROM. The disassembly phase decides: Code 0.7 or Data plus a remark. | 16 B fidelity |

After the batch, run map.validate, map.save and map.export, then a new checkpoint (e.g. pass-003). Then run
commentary on bank 0 $8547-$85B3 and $811A/$817A/$81C3, bank 2 $8363, bank 3 $85BB-$8615, and the bank 7 IRQ
header.

## Non-ready questions and deferred avenues
- **L-004, remaining 7 roots** (AttractAltScreen, DrawMenuLowerScreen, CL_DrawStaminaBoth,
  UpdateBestScoresHigh, SfxOnlyUpdate, MT_RankByDistance, DR_AdvanceCourse):
  - No absolute, word or word-1 reference exists in their bank or the fixed bank.
  - The multi-player runs did not execute them, and no selector is known.
  - They are classified already, so only reachability is open.
- **CL_Bracket4/5:** not run. They use the same NumPlayers selector that N=3 and N=6 demonstrated, so another
  run would repeat a known result. Recipe: mp_full.py / mp_pick.py with N=4 or 5, then mp_event.py.
- **S16, natural full Start-the-Games run:** deferred.
  - Final placings, results and Caves of Fame were visited in S10-S12. The S10 shortcut only set the event
    counter.
  - All six events have finished naturally.
  - A traced run is about 25k frames at about 4 fps (TK-002) and would only re-run observed paths.
- **L-005 (meanings of generic data):** semantic only; there is no code-vs-data question. This covers the
  bank 2 $8A9D block, bank 3 $AEF9-$B01D and the IrqSplitParam odd bytes.
- **Raster IRQ $C000:** no natural route. It would need I clear while bank 0/2/3/6 is mapped, and nothing
  supplies that.
- **Bank 4 $8376 `BIT $2002`:** a single unreferenced instruction, too little structure. Left as Data.

## Rejection review (compact; for coordinator inspection)
- **Claim "100% classified ⇒ no corrections remain":** rejected. Decisive artifacts are audit.txt (F hit at
  0xC620) and the extent arithmetic in notes.md §2. They show a 7-byte code fragment inside a table, so L-013
  is proposed.
- **Claim "Text regions in bank 0 are correctly bounded":** rejected for $8547-$85B3. The consumer's LDY #$14
  and the offset table (notes.md §3) rule out the current boundaries.
- **Claim "IrqHandler unreachable because only bank 1 CLIs":** the premise is wrong (L-015), but the
  conclusion survives on the corrected premise.
  - Strongest competing interpretation: an IRQ arrives after a bank switch while I is still clear.
  - Constraint against it: WarmStart and EnterNextEvent execute SEI.
  - No observation contradicts this.
- **Claim "L-004 roots unreachable":** never made. Two of them were shown reachable naturally. For the rest,
  only the absence of a known selector is recorded.
- **Generic data at confidence 0.7:** no code-vs-data contradiction found.
  - Checks run: fall-in and island scans, the JSR-to-known-entry scan, and fetch overlap.
  - Strongest remaining candidates were all examined and dispositioned: L-017, L-018, and bank 4 $8376.

## Artifacts
analysis/redirect/r02-a1/{notes.md, audit.py/.txt, fallin.py/.txt, islands2.py/.txt, mp_*.py, covcheck.py, r.py,
cov_mp3.json, cov_mp6.json, cov_new_r02.json, mp3_mainmenu.json, mp6_mainmenu.json, mp_result.json, frames/,
leads.before.json, scenarios.before.json, update_leads.py}. analysis/leads.json (L-013..L-018 added; L-004/L-005
attempts appended; pass record), analysis/scenarios.json (S15 visited, S16 deferred), analysis/progress.md.

Toolkit: no new issues found. The trace slowdown is the existing TK-002. No background processes are left
running: the bridge and serve were stopped.
