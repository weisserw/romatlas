# Round 2 redirect handoff

- Status: **COMPLETE** (one ready lead; not terminal). Input checkpoint **pass-002**
  (maps/pass-002.json = maps/current.json, byte_identical). Map, state, ROM, source unchanged
  (state.json/current.json sha256 b10f5901...).
- Classification at input: 100.00% rounded (131082 / 131088; 6 bytes unknown).
- Attempt dir: [analysis/redirect/r2-a1/](../redirect/r2-a1/) (README.md lists scripts/outputs).
- Proposed bytes: 6 (L-015, low confidence). Accepted: 0 (disassembly decides).
- Runtime: own bridge on /tmp/ra55bc_r2redir.sock (stopped at end); in-memory tracing only.

## Avenues and outcomes

1. **Residual 6 bytes (commentary Q1) -> L-015 ready.** Round-1/round-2 rejection
   ("isolated return opcode insufficient") was reviewed against its decisive artifact
   (r1-a1 small.py/xref.py: reference search only). New combined constraints:
   - No data consumer in any bank context ([refs.py](../redirect/r2-a1/refs.py) ->
     [refs_out.txt](../redirect/r2-a1/refs_out.txt), 14047 classified instructions): no
     branch/JSR/JMP target, no absolute operand; every indexed read whose window covers a target
     has its base in an earlier bounded table and would read across Observed code; no word or
     word-1 value in any non-code byte maps to a target; [indirect.py](../redirect/r2-a1/indirect.py):
     the only indirect transfer is JumpTableDispatch ($CA39, inline .word tables), no pha/pha/rts.
   - Not fill: all 16273 Padding bytes are $FF ([idiom.py](../redirect/r2-a1/idiom.py)).
   - Exact code-stream boundaries: each follows an unconditional transfer ending classified code
     (jmp $C360 / rts / jmp $C156 / clc;rts / jmp $CC10) and precedes an established instruction
     start ($C406, $C917, $CA5C, $A1E5) or, for $9B8B, data JumpArcPtrs.
   - In-ROM precedent: the only two classified "rts right after an unconditional transfer"
     instances are referenced null-handler stubs ($E3B2 after rts; $FAFE `ReturnOnly` after
     jmp $CC46, a jump-table entry). Bank-6 player code uses paired carry exits
     ($99EC/$99EE, $9A26/$9A28, $B9BF/$B9C1; [secrts.py](../redirect/r2-a1/secrts.py)); $9B8B sec;rts
     sits right after ArcStep_Return clc;rts. $C405 matches State08_StageExit's "sub-state block
     ends in rts" template; $CA5B/$A1E4 match jsr X;rts -> jmp X leftovers; $C916 is a double rts.
2. **Low-confidence L-003 fragments (Q2): keep, no demotion.** [l003.py](../redirect/r2-a1/l003.py):
   all 14 end exactly at existing instruction starts; every jmp/jsr/branch target inside them lands on
   an existing classified instruction start in the same bank context; target-free ones use PPU/RAM
   idioms (PPUCTRL via shadow, PPUSTATUS+PPUADDR, $0141 flag read by the next routine, skip-entry
   load); bank-6 $B8F2/$B8F5 jump exactly to the instructions after the $C312/$C31E prefixes. No data
   consumer. Confidence 0.6 and "no known caller" wording stay.
3. **Boss visit: SC-BOSS6 visited** (analysis/scenarios.json): stage-clear warp from
   scratch/r2/snap_s3orb.json with $28=5, then half-1 restart at StageCameraLockTable[6]=$10; boss
   slot 7 type $1B = ObjT1B_Medusa confirmed; fight, defeat, orb, clear, stage 7. 3696 traced starts,
   all at existing classified instruction starts (113 in bank 10); no residual byte executed.
4. **Q3 stage/spawner half-list lengths: deferred** - naming-only (no structural effect); boss identities
   for 3/6/18 now visited; 9/12/15 recipe recorded (SC-BOSS9-12-15, lock values $05/$03/$10) but
   their visits only confirm tentative names.

## Ready leads

| ID | Range (physical / CPU, context) | Proposal | Bytes |
|---|---|---|---|
| L-015 | 115733 $C405; 117030 $C916; 117355 $CA5B (fixed); 105371-105372 6:$9B8B-$9B8C; 106996 6:$A1E4 ([12,13,14,15]) | Code, Static, confidence ~0.5, "unreferenced return stub"; reachability unknown | 6 |

Recipe: map.annotate each range as Code/Static with the neighbouring regions' context, evidence
pointing to refs_out.txt and the precedent census; keep separate regions; re-run refs.py on the new
map (expect identical empty result); checkpoint pass-003; commentary names them as unused return
stubs with explicit uncertainty. Reject only on a concrete data consumer or fill convention.

## Rejection review (for the inherited round-1 R3 rejection)

- Claim: the 6 bytes are code (return stubs), not data.
- Decisive prior evidence: absence of references; it rules out a known caller, not a code role.
- Independent constraints combined: no reader/pointer in any bank context; not $FF fill; exact
  instruction boundaries inside code streams; in-ROM precedent of identical lone-rts stubs used as
  null handlers; paired carry-exit idiom for $9B8B.
- Strongest competing interpretation: leftover/unused bytes of no role (deleted code or stray data);
  deleted-code leftovers are still code-origin, and a data role has no consumer, format or fill support.
- Result: supported low-confidence proposal (L-015); reachability stays unknown, not Observed.

## Deferred / unchanged

- L-004/L-005/L-006 semantic only; L-008 toolkit-blocked (TK-NES-004); no new toolkit issues.
- Callers of unreferenced routines (L-003, $90A1, $910A, $CF61, $FF06): none found; reachability unknown.

## Recommendation

**Another disassembly round** for L-015 (6 B -> 131088/131088 if accepted), then commentary
(names/notes for the stubs) and final acceptance (editorial audit). If the reviewer rejects L-015,
the 6 bytes stay unknown with this review as the terminal record.
