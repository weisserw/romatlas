# Round 1 — redirect phase handoff (attempt r001-a)

Status: complete. Recommendation: **next: disassembly round 2** (ready leads L-007..L-014).

- Input checkpoint: pass-001 (byte_identical). ROM classification unchanged: 61.07%
  (240154 / 393232; 153078 unknown). Redirect changed only research/lead/scenario records;
  ROM, map, state, commentary source and configured binary untouched.
- Research dir: analysis/redirect/r001-a/ (notes.md = pipeline + negative results, tools/,
  candidates.json, residual-after-candidates.json, evidence/, snapshots/). Leads: analysis/leads.json
  (pass record "redirect-r001-a"; previous leads preserved with appended attempt history).

## Scope
Commentary questions 1-7 plus the uncommented unknown gaps: FarCall tables/targets, every
jump-engine site, calls into unclassified bytes, bank-31 tables, banks 0/4/6 role, orphan
gaps, bulk data consumers; runtime: 1P game end (win and loss), rating/level screens, next game,
EDIT re-run.

## Findings
1. FarCall tables (Q1): all 52 jsr/jmp $EF38 sites scanned. Each even bank's table extent is proven
   twice: by the used indices and because the lowest entry equals the end of the table. Bank 22
   entries 10 and 22 point to single RTS bytes. Bank 6 has a 1-entry table but no caller.
2. Static closure (Q4, Q6, L-001): roots = pass-001 code, FarCall entries and bounded inline jump
   tables (46 tables; each ends at its lowest target and/or at known code). This adds 9.2 KB of
   reachable code and 692 table bytes, with 0 overlaps or conflicts. All 16 FarCall sites inside
   unknown bytes fall inside it. In artificial game-end runs, 546 instruction starts landed in it
   and none in the remaining unknown bytes. This covers: $B93D (cold boot), bank 23 $A1DB/$A1F1/$A58B/$A66B
   and bank 22 $994D.
3. RunMode tables $EAC7/$EAFB (Q2): 4 words each; entry 0 points to the RTS at $EACF/$EB03. Other
   bank-31 data (Q3) is bounded and identified, and the internal cartridge header at $FFE0-$FFF9 is
   confirmed by checksums (L-011).
4. Banks 0/4 (Q5): **refuted** as stadium banks. Tbl_26_82AB gives bank 2 for every stadium,
   and nothing else selects R6/R7. Banks 0/4 are unreferenced copies of the bank-2 field
   code (9 operand bytes differ) with their own field data (L-010, L-012). Commentary
   "Stadium -> bank 0/2/4" must be corrected.
5. Dead code: bank 26 Stadium dispatchers ($93BA, $996C) and their variant tables were bypassed by a
   direct call ($93AB -> $93B7). Bank 31 has math-library routines with no caller. Bank 6 is an
   unreferenced older screen bank similar to bank 16 (L-009, L-014).
6. L-004 resolved: the always-taken branches lead to data tables.
7. Gameplay (Q7, L-005): with RAM recipes, a 1P win reaches GAME SET, the level screen, the RATING
   SCREEN and the next opponent. A 1P loss reaches GAME OVER (S-009, S-010). The run totals
   $0383/$0394 drive the end check (static check at bank 20 $8326).

## Ready leads (recipes in analysis/leads.json)
| ID | Scope | Expected bytes |
|---|---|---|
| L-007 | FarCall word tables at $8001, 13 banks | 76 |
| L-008 | candidates.json group main (excl. bank 6): code + 46 jump tables | ~9,900 |
| L-009 | orphan coherent routines (bank 26 variants, bank 31 math, small) | ~1,318 |
| L-010 | banks 0/4 code copies + Tbl_26_82AB | ~3,560 |
| L-011 | bank 31 tables, RTS handlers, internal header | 187 |
| L-012 | field data bank 2 (+0/4) with consumer-anchored sub-blocks | ~38 KB |
| L-013 | bulk residual generic data with anchors (exclude code-suspect chunks) | up to ~95 KB |
| L-014 | bank 6 dead screen bank (frontier $8387) | ~1.6 KB code + ~14 KB data |
Suggested order: L-007, L-011, L-008, L-009, L-010, then recompute residual (tools/segment.py)
and do L-012/L-013, then L-014. Candidate ranges list the effective prg_banks for each record.

## Unknown complement ledger
The remaining unknown ranges are in analysis/redirect/r001-a/residual-after-candidates.json, as computed
after the L-007..L-011 candidates: 104 gaps, 138,241 bytes. Each gap has its bank, CPU address and
code-suspect byte count. Planned handling: data gaps go to L-012/L-013, bank 6 to L-014, and
code-suspect chunks (4,578 bytes) stay unknown until someone inspects them.

## Deferred avenues (reasons)
- 1P later opponents/ending: the missing fact is the writer of HomeTeam/the progression counter.
  A static scan found no absolute store except the initialisers. The Write trace does not record
  the writing PC (TK-004), and two replay-to-cycle attempts did not reproduce the write, so I stopped.
  Recipe: L-005 "next" (snapshot gwin_post).
- 2P mode: the attempt went to EDIT because of a wrong cursor assumption. The correct route is in
  S-011. Low value: the 2P menus share bank-20 code that is already covered.
- Semantic meaning of the bulk data (music/nametables/rosters): this is not needed for generic data classification.
- The 1P progression counter and $0356/$0357 meanings were not resolved (they are scoreboard
  statistics, not progression).

## Toolkit
TK-004 added (Write events lack the issuing PC; trace summary counts ignore filters). Earlier TK-001..003 still apply.
Scratch /tmp/ra67r removed at handoff; serve daemon stopped.
