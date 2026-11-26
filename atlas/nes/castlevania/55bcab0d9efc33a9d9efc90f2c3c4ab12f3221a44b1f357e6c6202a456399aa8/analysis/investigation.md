# Investigation ledger

## Round 1 disassembly (checkpoint pass-001)

Game: Castlevania (US), UxROM, 8 x 16K PRG, CHR-RAM. Fixed bank = 16K bank 7 ($C000-$FFFF,
8K banks 14/15). Effective PRG contexts use 8K bank indices.

### Methods and evidence sources

1. Observed execution: tracing (capacity 0) during natural startup/title/attract/stage-1 play and
   artificial RAM-patched scenarios (stage warps, game-state probes); contextual roots dumped by
   scratch/getcov.py (6296 instruction starts, 4526 natural). Observed regions say whether natural
   play was included.
2. Static closure: scratch/sa2.py (local helper; toolkit static discovery stops at every JSR on
   mapper 2, TK-NES-004). Rules: official opcodes only; JSR returns unless callee pops the return
   address (JumpTableDispatch $CA1E); deterministic branches after immediate loads/CLC/SEC and
   complementary branch pairs only follow the taken side; no instruction overlap; data barriers.
   $8000-window bank context comes from (a) the caller's own 16K bank, (b) `ldy #n; jsr $C195`,
   (c) `jsr $C193` = bank $24, which is written only at Reset ($C02B, A=6) so it maps bank 6.
   Fixed-bank code reached from observed roots does NOT inherit the observed bank (rejected method,
   see below). Inline jump tables after JSR $CA1E: entries read until a non-pointer word, existing
   code, or the lowest in-bank target after the table; banked entries of fixed-bank tables are
   resolved only when all contract-derived arrivals and observed contexts at the dispatch agree
   ($C1AE GameStateTable and $F3B6 table: bank 6).
3. Data: (a) 8K banks 1-9 and 11: no execution, no static entry, no code-like run >= 40 bytes
   (scratch/islands.py), repeated tile/metatile patterns -> generic Data; uniform $FF bank tails ->
   Padding. (b) Gaps inside code banks -> generic Data when they start at an absolute/indexed data
   operand base used by classified code, or when linear decode from the first byte fails within
   8 bytes (unofficial opcode, BRK, implausible operand: no PRG-RAM on this board) and no code-like
   run >= 12/24 bytes exists; the remainder after a code-like island stays unknown.
   (c) Specific: iNES header, BankSelectTable $C000 (8), IdleRngTable $C03F (16, AND #$0F bound),
   vectors, NotePeriodTable (bank 0, 12 equal-temperament periods), 16 inline jump tables,
   return-address words $E9A4 and $F3F9 pushed before dispatch.

### Rejected / corrected during the pass

- R1: propagating an observed fixed-bank root's $8000 bank along static paths. Counterexample:
  $F890 observed with bank 0, but `jsr $A08A` at $F8A0 only ever runs with bank 6; static bank-0
  propagation decoded garbage (bad opcode at $A09A bank 0). Removed; fixed roots carry no bank.
- R2: resolving banked entries of fixed-bank jump tables with the bank of whichever arrival came
  last (produced ~6 KB of bogus bank-10 "code" in an early scratch run). Replaced by the
  all-arrivals-agree rule above; nothing from the bogus run was annotated.
- R3: gap-data rule without an early-decode-failure test classified two code+jump-table fragments
  as data (0x140D7 "jsr $EE83; lda $046C,x; and #$7F; jsr $CA1E ..." and 0x19B9B "sec; rts").
  Both regions were removed; 0x19B9D.. re-added as data (operand-referenced), 0x19B9B-C and
  0x140D7.. left unknown. Rule tightened for all later gap batches; earlier batch re-checked.

### Remaining unknowns (8621 bytes)

- 8K bank 10 (16K bank 5 $8000-$9FFF): 6912 B, mostly code-like islands (lead L-001).
- 8K bank 12: 694 B; bank 13: 20 B; bank 14: 502 B; bank 15: 486 B; bank 0: 7 B
  (leads L-002, L-003). Full list: `analysis.unknown` on analysis/state.json.

### Lead outcomes

No leads existed before this pass. Produced L-001..L-008 (analysis/leads.json).

## Round 1 redirect (attempt r1-a1, input pass-001)

Static research only; evidence and scripts in analysis/redirect/r1-a1/ (proposals.json,
closure_out.json, README.md). Handoff: analysis/handoffs/round-1-redirect.md.

- Bank 5 handler context: banked ObjectHandlerTable entries decode as handler entries only in
  bank 5 (bank 6 decoding hits data/unofficial opcodes); fixed wrappers $FCCC-$FCF3 restore
  bank 5 after PlaySound. Static arrivals with bank 6 (State08/09/0B) exist but are not a valid
  handler context. Full bank-5 closure: 2780 instructions, 0 conflicts; residual bank-10 bytes
  are operand-anchored tables and the demo scripts ($9A38/$9B64/$9CCF). -> L-001, L-014.
- Correction found: $EBF6-$EBFC (pass-001 generic Data) is a code entry called from bank 5.
- Banks 1-9/11 whole-bank data: confirmed (bank selects only 0/4/5/6 with fixed-bank callees;
  loader $FD21 copies data; no transfers into those banks; no observed fetches).
- Unknown ledger dispositions (8621 B): bank 10 6912 -> L-001; fixed 226 -> L-001; $D87B 420
  -> L-009; bank 6 $90A1 683 -> L-010; $FF06 244 -> L-011; $CF5F 29 -> L-012; 14 B -> L-013;
  87 B dead-entry fragments -> L-003 (low); 6 B lone RTS / sec;rts -> no proposal.
- Bank-6 code at $82A2/$84FF/$88B7/$8C9A independently supported ($F3B6 table, observed,
  fall-through); not an artefact of the bank-5 jumps.

## Round 2 disassembly (input pass-001, output checkpoint pass-002)

Applied verified redirect r1-a1 proposals with scratch/r2/apply.py (123 map.annotate calls, 0 failures,
map.validate ok). Before annotating: re-ran analysis/redirect/r1-a1/closure2.py on the pass-001 map
(identical result: 2780 instructions, 0 issues; only conflict = known $EBF6 Data) and entries.py
(banked ObjectHandlerTable entries decode as handler idioms in bank 5, data/unofficial opcodes in bank 6);
spot-decoded $FA90 (copies 32 B from $FAAB -> data $FAAB-$FACA, not in proposals.json, added),
$90A1/$910A (bank 6), $CF61, $FF06 and the DMC sample bounds ($4012=$FD -> $FF40, $4013=$0B -> 177 B).

- Structural evidence vs reachability: all new code is Static. L-001 code has static entries (handler
  table, dispatch tables, direct calls); L-010/L-011/L-012 routines and L-003 fragments have no known
  caller (reachability unresolved, not claimed dead). L-003 fragments use confidence 0.6.
- Runtime cross-check (artificial boss/ending scenarios SC-BOSS3, SC-BOSS18, SC-ENDING2): 4139
  observed instruction starts, 623 new vs round 1; every one lies in an existing Code region (all
  Static), none in Data/Padding/unknown. Bank-10 observed starts (213) all coincide with closure
  instruction starts and run with context [10,11,14,15]. Provenance left Static (regions not split).
- Kept unknown (6 B): lone $60 bytes $C405, $C916, $CA5B, bank-13 $A1E4 and bank-6 $9B8B-$9B8C
  (`sec; rts` after a `clc; rts` return); an isolated return opcode is insufficient structure.

Per-lead outcomes: L-001 verified (7138 B), L-014 verified (corrections), L-009 verified (420),
L-010 verified (683), L-011 verified (244), L-012 verified (29), L-013 verified (14), L-003 verified
(87, low confidence), L-002 superseded (by L-010/L-013), L-007 verified (bosses + ending visited).
Remaining unknowns: the 6 bytes above.

## Round 2 redirect (attempt r2-a1, input pass-002)

Evidence in analysis/redirect/r2-a1/ (README.md). Handoff: analysis/handoffs/round-2-redirect.md.

- Residual 6 B ($C405, $C916, $CA5B, bank-6 $A1E4 lone rts; bank-6 $9B8B-$9B8C sec;rts): reopened
  with combined constraints instead of "references only". No reader in any bank context (14047
  instructions: no absolute operand, indexed windows only from earlier tables across Observed code,
  no word/word-1 values, only indirect transfer is JumpTableDispatch, no RTS-trick dispatcher); not fill
  (all Padding is $FF); exact code-stream boundaries; in-ROM precedent: the only two classified
  rts-after-unconditional-transfer instances ($E3B2, $FAFE ReturnOnly) are jump-table null stubs;
  $9B8B mirrors the bank-6 sec;rts/clc;rts paired exits. -> L-015 ready (Code, Static, low confidence;
  reachability unknown).
- L-003 fragments: kept (all control targets land on existing instruction starts in matching banks;
  target-free fragments use PPU/RAM idioms consistent with neighbours).
- SC-BOSS6 visited: stage 6 boss is type $1B (ObjT1B_Medusa confirmed); 3696 traced starts all at
  existing classified instruction starts (113 in bank 10); no residual byte executed.
- Commentary Q3 (stage spawner half-list lengths) deferred: naming only, no structural effect.

## Round 3 disassembly (input pass-002, output pass-003)

- L-015 independently reviewed and accepted. Bounds: each residual sits between an unconditional
  transfer ending classified code (jmp $C360 / rts / jmp $C156 / clc;rts / jmp $CC10) and an established
  instruction start ($C406, $C917, $CA5C, $A1E5) or the absolutely-addressed table $9B8D. No data consumer
  (r2-a1 refs_out.txt; $A1E3 word-1 hits are 8K bank 0/1 data with no RTS-trick dispatcher in the ROM), not
  $FF fill, return-only stub precedent ($E3B2, $FAFE) and bank-6 paired carry exits. Annotated Code, Static,
  confidence 0.5 (scratch/r3/apply.py); evidence records structure and "reachability unknown" separately.
  refs.py re-run on the new map: identical empty sets (scratch/r3/refs_after.txt).
- Result: 131088 / 131088 bytes classified, 0 unknown. Handoff analysis/handoffs/round-3-disassembly.md.
