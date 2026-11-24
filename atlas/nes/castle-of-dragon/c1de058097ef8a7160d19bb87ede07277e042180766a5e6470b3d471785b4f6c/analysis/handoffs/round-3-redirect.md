# Round 3 redirect handoff (attempt r3-a1)

- Attempt dir: analysis/redirect/r3-a1/ (tools/{staticreach,edgeconflict,datarefs,dispatch,fielddf,immpairs}.py,
  evidence/{static_reach.json,audit_output.txt}, leads.before.json). r1-a1 and r2-a1 untouched.
- Input checkpoint: pass-003 + round-3 commentary source (byte-identical). No map/source/ROM edits; no emulator runs,
  no processes started; no scratch created. input/game.nes sha256 c1de0580... unchanged.
- ROM classification unchanged: 100.00% (131087/131088). Exact unknown complement recomputed from analysis/state.json:
  one byte, phys 0x1C425 = CPU $C415 (fixed bank 7); no overlaps. Proposed bytes this pass: 0. Accepted: n/a.

## Methods (static only, on analysis/state.json + input/game.nes)
1. Static-code audit (new): decode all 192 Static Code regions (2769 bytes) instruction-aligned with bank context
   (bank 6 at $8000, fixed bank 7 at $C000; banks 0-5 contain no code). Reachability from all Observed instruction
   starts plus words in bank-6/7 Data regions equal to a start or start-1 (RTS trick); branch/JMP/JSR/fall-through edges.
2. Edge-conflict check: every branch/JMP/JSR target in all Code regions must land on a Code instruction start.
3. Indirect-dispatch enumeration: JMP (abs), PHA/PHA/RTS sequences, task-stack page writes.
4. Immediate-pair reference scan (new form not covered by r1/r2): #<T and #>T immediates within 12 instructions, for
   T = $C415/$C414 and every L-002 start / start-1.
5. ObjFieldD/F consumer check: all absolute(-indexed) operands $03xD/$03xF and all (zp),y pointers.

## Findings
- Static code is structurally sound: official opcodes only, no straddles, 0 edge conflicts. 184/192 Static regions
  (2466 bytes) are reached from Observed code or dispatch tables (ObjHandlerTbl_9632/_A651 via JMP ($0530), IRQ vector
  for $C0D8, sound RTS-trick table). The 8 unreached regions (303 bytes) are exactly the L-002 fragments
  $A38F,$A6E8,$A870,$B44B,$C1D5,$C1EC,$CCE6,$D375 ($9A27 is table-referenced). No doubtful code claim found.
- Indirect transfers: JMP ($0530) at $962E/$A63F/$AC34, JMP ($06DE)/($06E0) at $C10A/$C0EB, RTS dispatch $CC3E,
  scheduler stack writes $C035..$C220 (CreateTask takes entry-1 from frame args). None of their sources, nor any
  immediate pair, nor any bank-6/7 table word, yields $C415/$C414 or an L-002 start.
- ObjFieldD/F: only 6 absolute writes ($E214,$E21A,$E32F,$E335,$E344,$E34A) plus whole-record copies (cutscene init
  table; guardian mirror copy loop ObjY,y for $20 bytes). No individual read; (zp),y pointers are SndDataPtr/SndEnvPtr
  only. Write-only fields: their meaning cannot be recovered from code. Editorial note only (L-011).

## Leads (analysis/leads.json)
- READY: none.
- L-011 new, verified (audit + D/F check, no structural change; wording note deferred to final editorial audit).
- L-001 attempt appended (still blocked). L-002 attempt appended; closed for redirect (reachability has no method).

## Deferred / rejected avenues
- L-002 reachability: all in-ROM reference forms now exhausted (absolute, relative, table, RTS-trick, split/immediate
  pair, indirect via RAM vectors). Natural reachability would need a reference form not present in the ROM; more play
  only re-tests already-negative traces. Rejected: no method.
- Remaining 2466 table/branch-reached Static bytes: classification already supported; play would change provenance
  only (round-2/3 disassembly judged diminishing, TI-003 trace cost). Deferred: no structural question.
- Object/sound/item identity names, stage 5/6 cut-content intent, MsprFrameData $8A1D-$8A40 use: semantic only, inside
  classified regions; no consumer exists to test (r2-a1 finding 2, 6). Rejected: no discriminating in-ROM test.
- Natural unpatched play-through: all milestones already visited (scenarios.json incl. ending, staff roll, bosses,
  test mode). It cannot touch $C415 or L-002 (no in-edges). Deferred: no structural prospect.

## Rejection review ($C415, phys 0x1C425, bank 7 fixed; the only unknown byte)
- Candidate claim: $C415 ($60) is code (orphan RTS stub) - or, alternatively, generic data/padding.
- Decisive evidence: $C414 is the real end of ClearSpritesAndObjects (beq at $C40E targets it); $C416 InitFramePool is
  a Reset JSR target. r1: no absolute/RTS-trick/split-table word; r2: 1168 branches + all JMP/JSR/JMP-ind operands, no
  target; r3: no immediate-pair reference, no dispatch source (all indirect transfers enumerated), every code edge lands
  on known instruction starts; never fetched in any traced scenario (r1-r3, incl. forced/artificial ones).
- What it rules out: any static entry into $C415 by every reference form present in this ROM. It does NOT discriminate
  "dead RTS stub" from "stray data byte": both predict exactly this evidence.
- Independent constraints together: (a) no fill role - bank fill is $00, `60 60` occurs nowhere else in code;
  (b) no alignment role - $C416 is not aligned; (c) no record/table structure - a single byte between two code routines,
  no consumer; (d) consistency with L-002: those fragments were accepted as code on multi-instruction contracts (known
  variables, scheduler/sound conventions, gap-fill between table handlers); $C415 has no contract beyond one opcode.
- Strongest competing interpretation: leftover RTS from an edited routine (code) vs stray/padding byte (data). The policy
  example states an unreferenced return opcode lacks sufficient structure on its own; the data interpretation has no
  positive evidence either (no repetition, uniform run or table).
- Result: no supported proposal and no revised test exists in the ROM; discrimination would need external
  source/toolchain evidence. Keep unknown. Premises re-checked against primary evidence; no overbroad scope found.

## Recommendation
(c) terminal incomplete. Classification 99.9992% (131087/131088); residual $C415 unknown per review above. Code
Observed 21962 / Static 2769 (structurally audited). Commentary complete; final editorial audit still pending
(including the optional ObjFieldD/F wording from L-011). No processes left running; no scratch to clean.
