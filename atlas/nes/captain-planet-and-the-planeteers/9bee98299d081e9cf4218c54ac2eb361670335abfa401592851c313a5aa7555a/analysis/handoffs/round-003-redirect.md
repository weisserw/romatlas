# Round 3 redirect handoff (attempt r003-a, input checkpoint pass-003)

**NEXT=terminal** (incomplete: 4 Unknown bytes; no ready leads). Research only: ROM, map, state, source, toolkit and
binary unchanged. Record: analysis/redirect/r003-a/README.md.

Classification: 262156/262160. Residual (unchanged ledger r002-a + pass-003 split): bank 6 $A175 ($0C185), $A18B ($0C19B);
bank 15 $FB69 ($1FB79), $FDC0 ($1FDD0) - each one $60 directly after a routine's RTS and before classified code.

## Avenues and findings
1. Residual 4 bytes (L009 residual) - reviewed, no proposal. consumers4.py (all 4 sites), module-copy search, fill survey,
   new coverage: no reference/target-1 word/imm pair/indexed reach (indexed and raw-word hits are bounded known tables or
   bank 13's own ObjectAnimLists13); no shared-module counterpart (bank 3 PlaceShot copy is followed by ordinary code);
   PRG fill is $00/$FF only; none fetched (cov_r003a.json, 4605 offsets, 0 outside Code).
2. Gameplay (analysis/scenarios.json, all artificial setup):
   - Level-0 boss zone visited; GoalTargetTask defeated by 6 REAL A-button special-move hits after teleporting the target
     onto CP -> GoalTask clear, score 50, victory pose, WELL DONE (snapshot r003-a/l0boss_zone.json).
   - Level-1 goal defeat: BarrierBuilderTask (5 parts $B732) + minion $B6A1 observed; BossPartsHit=5 -> stage clear.
   - Level-2 CP stage gameplay traced; level-2 GoalTask "MapColumn<4 = reached" rule observed; next stage started.
   - 153 Static Code regions gained executed opcodes (1744), zero contradictions. L008 static-only routines still unexecuted.

## Ready leads: none (leads.json pass redirect-r003-a: added [], updated [L009]).

## Deferred
- Barrier parts by play (power 2): power-select RAM contract not established; outcome already observed by patch.
- Played level-3 boss defeat / CpHurtCount>=$10 exception: semantic only; no byte or classification depends on it.
- L008 reachability: no reference, no fetch in any route of 3 rounds; no new method.

## Rejection review (terminal)
- Candidate claim: each of $0C185/$0C19B/$1FB79/$1FDD0 is Static Code (dead/stub RTS) OR generic Data (padding).
- Decisive evidence: r003-a/tools/consumers4.py output (no consumer), r002-a/tools/rtsrts.py (5 rts-rts pairs; only bank 12
  $BF0A's 2nd RTS is referenced, as a branch target), PRG uniform-run survey (fills $00/$FF), cov_r003a.json + cov_r002a.json.
- What it rules out: any decoded-code reference/dispatch to the bytes (incl. RTS-trick target-1 and bank-13 coincidental words),
  alignment padding, this ROM's fill convention, a shared-source module copy. It does NOT rule out an original-source dead
  `rts` (removed caller / disabled entry) or a stray assembler byte - these two remain indistinguishable.
- Independent constraints together: code context on both sides + RTS opcode + no reference + unaligned successor. All are
  consistent with both readings; none constrains the byte's role beyond legal decoding.
- Strongest competitor: dead duplicate RTS (code). Data reading has no positive support (single byte, not fill, not a pattern,
  decodes as an instruction), so the generic-data fallback in evidence-policy does not apply; code reading is exactly the
  "unreferenced return opcode on its own" case the policy rejects. Bank 12 $BF0A precedent is an analogy, not a constraint.
- Outcome: no supported proposal and no revised discriminating test (only a reference or a fetch would decide, and searches
  of all dispatch mechanisms plus 3 rounds of coverage found neither). Remain Unknown; stop incomplete with exact residual 4 B.

Server stopped; r003-a 5.0 MB (one cited snapshot); disk ~3.1 GB free. No toolkit issues.
