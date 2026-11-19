# Round 2 redirect handoff (attempt r02-a)

- Input checkpoint: pass-002 (maps/current.json unchanged). Classification unchanged: 100.00%
  (393223 / 393232; 9 B unknown). Redirect changed only research/lead records: analysis/leads.json
  (L006, L017, L018 updated; L019 new; pass record appended), analysis/scenarios.json (+2 milestones),
  analysis/progress.md. Prior attempt analysis/redirect/r01-a/ untouched.
- Research dir: analysis/redirect/r02-a/ (tools/, evidence/, leads.before.json, scenarios.before.json).
- Status: READY leads for one more disassembly/commentary cycle (L017 classification proposal, 9 B;
  L019 commentary batch). Not a terminal handoff.

## L017 - the 9 residual bytes (ready, proposal: Code / Static / dead tail, reachability none)
Ranges (physical, bank, CPU): 0x08743 b4 $8733 RTS; 0x0B1BE b5 $B1AE RTS; 0x0B7A2 b5 $B792 RTS;
0x14CCA b10 $8CBA RTS; 0x17BC6 b11 $BBB6 RTS; 0x1C1E2-0x1C1E4 b14 $C1D2 JMP $FBCC (reset entry);
0x1FD5A b15 $FD4A RTS. Pair context = own even/odd pair; 14/15 fixed. Ledger: evidence/unknown-ledger.json.
Method (tools/l017.py, `p2` mode for reads):
1. Bank-aware reference audit over every instruction start of every Code region (branches, abs/ind
   operands; fixed-bank code tested against all 7 pairs; operand+1), word and word-1 search across PRG,
   immediate lo/hi pairs -> 0 references. 9 raw word hits are straddles (e.g. inside the $CE4C table at an
   odd offset, opcode+operand coincidences) - checked individually (evidence/l017-refs.json).
2. Data-consumer audit: no abs/indexed read covers any byte (nearest indexed tables $B6B3 7 B, $BAD1 8 B
   end earlier). The ROM is otherwise fully classified, so every potential consumer was in the audited set
   (evidence/l017-reads.json).
3. Idiom census: terminal JMP + RTS occurs 60x; all 55 classified ones are referenced exits; the 5 L017
   RTS are the only unreferenced ones. In 3 cases (0x08743, 0x0B1BE, 0x0B7A2) a branch in the same routine
   targets the instruction just after the dead RTS (`bxx Skip .. jmp X / rts / Skip:`) - the shape left by
   converting `jsr X / rts` to `jmp X`. Fill regions never use $60/$4C.
4. Runtime: final boss defeat -> StageEvt_FinalBossCleared -> Ending_Run executed, the dead tails of
   exactly those routines (0x0B7A2, 0x1C1E2) not fetched (consistent, not proof).
Recipe: annotate each range Code, Static, confidence <= 0.6, evidence "unreachable dead instruction tail;
no control/pointer/data reference in the fully classified ROM (r02-a)"; never Observed/entry. Split
acceptance allowed (0x1C1E2-4 strongest). If rejected, keep unknown and record the reason (terminal residual).

## L019 - commentary batch (ready, no map change)
- ObjType $33 runs in pair 6/7: its own state table $EE87 already lists pair-6/7 handlers; only in 6/7 are
  $8754 and $BE86 instruction starts. $8754 = ObjT03_Spawn (bank 6), $BE86 = ObjT32_Handler_St00_St01_Skip
  (bank 7; ObjT33 is a fixed-bank twin of ObjT32). Runtime (artificial trigger, stage $11): BossClass 0,
  fetches at bank 6 $8754 / bank 7 $BE86, none at 4/5 or 10/11 aliases (evidence/probe-objt33.json).
- ObjType $32 (records $970F/$9718) and $33 ($973C) come only from the stage $11 wave script $96A3
  (phys 0x196B3-0x1975D). Wave record = 9 bytes [column, fine X, packed pairs/group, StageScriptNext,
  WaveId->ObjType, chr set, palette, group data, StageScriptCmd]; StageScriptPos += 9 (evidence/wave-scripts.json;
  record parsing past each script's end is unvalidated - terminator not established).
- StageEventIndex 3 (StageEvt_Stage3_1A) is consumed in StageState 0/1: StageIntro_Load skips the intro
  screen when non-zero and StageLoadGraphics ($8370.., bank 4) takes the "other -> column 0, pos 0" path
  and clears it before StageState 4 (StageEventDispatch) can run; StageEventByIndex[3] is never read.
- Loop-1 slots under BossClass != 0: only boss types ($0B,$19,$1F,$28 by probe; $2D,$34 from the
  commentary typeprobe) - no non-boss type observed under pair 4/5 or 10/11 (bounded; probe-bosspair-partial.log).

## L006 (verified, artificial assist) / L018 (blocked)
- Final-boss attack contract: player slots $10/$11, boss slot 0/1; same ground line (ObjY+1) and
  |dX| <= 28 facing -> punch hits (-2 HP; 160 HP natural). Unassisted bot dies after ~2 hits (evidence/fight-unassisted.json). With
  $0145=8 and PlayerHealth refresh, real hits defeated the boss -> StageState 4 -> GameMode 7 -> ending
  steps; $B7B9 fetched; 0 fetches outside Code starts. Snapshot traces/round2/final_boss_defeated.snap.
- L018: 18 no-caller routines (442 B): 0 external references, 0 fetches in the r02-a runs; missing fact is a
  caller that does not exist in the classified ROM; classification unchanged (Static 0.7).

## Rejection review (L017, in case disassembly rejects it)
- Claim: 9 bytes are dead assembled instructions (Code, Static), not data; no reachability claimed.
- Decisive prior evidence: round-2 disassembly kept unknown because "no reference/fetch or supported
  convention" (analysis/handoffs/round-02-disassembly.md). r02-a confirms the no-reference part
  (evidence/l017-refs.json), which rules out reachable code only - it does not discriminate code vs data.
- Independent constraints combined: zero data consumers in a fully classified ROM (l017-reads.json);
  exact one-instruction gap between code regions of the same bank context; not a fill value; the
  JMP->RTS idiom (55/55 referenced elsewhere) plus branch-past-the-tail shape in 3 cases; JMP operand =
  validated reset entry.
- Strongest competing interpretation: an unconsumed stray data byte/remnant (no positive support).
- If rejected: no further test exists (no execution path, no consumer); the 9 B stay the exact residual.

## Deferred / not actionable
- Fully unassisted final-boss defeat: needs dodge play (enemy attack timing), not a RAM contract.
- Exhaustive boss-loop census: bounded sample sufficient for commentary; more stages give repetition.
- Wave-script terminator/extent per stage: not needed for classification (scripts already classified).
- Scratch: /root/rascratch-r02a (6 MB, disposable). No server left running. Single CPU: run probes serially.
