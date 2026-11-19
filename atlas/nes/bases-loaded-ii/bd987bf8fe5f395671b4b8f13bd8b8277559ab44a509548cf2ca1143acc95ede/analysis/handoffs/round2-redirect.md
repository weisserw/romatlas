# Handoff: round 2, phase 3 (redirect), attempt r2-redirect-a

## Scope and checkpoint
- Input: maps/round2-disassembly.json (sha256 8b43ff3e..., = maps/current.json) and
  commentary/source/game.asm (sha256 36b37cc1...). 393,222 of 393,232 bytes classified;
  **10 bytes unknown**. The exact complement was recomputed from the map and matches L-016.
- Records: analysis/redirect/r2-redirect-a/ (findings.json, unknown-ledger.json,
  deadrts-survey-strict.json, ptrsearch.json, runtime.json, tools/, experiments/).
  analysis/redirect/r1-redirect-a/ is untouched.
- Redirect changed only research and lead records. The map, state, source, ROM and toolkit are
  unchanged. No bytes were proposed as data. The proposed byte changes are listed below.

## Findings
1. **L-016 (10 residual bytes): reopened with newly combined constraints. Proposal: Code,
   Static, confidence 0.5, unreferenced.**
   - **Gap census (tools/deadrts2.py, short-gap census).** Every other short non-code gap
     between classified code has a consumer: an inline table or a jump table. The one
     exception is the 1-byte $20 filler at bank 0 $8420. No other unread data byte sits
     inside a routine anywhere in the ROM.
   - **Byte value.** All 7 sites end in $60 (RTS). The ROM's fill is $FF (83,258 bytes) or $00
     (40 bytes), so these bytes are not fill.
   - **Position.** Each site follows an unconditional transfer: JMP (5 sites), RTS (bank 15
     $D9F4), or a dispatcher JSR plus inline table (bank 11 $C286). Three sites lie strictly
     inside a routine body, with the routine's own forward branch jumping over the byte:
     bank 6 $CA2E, bank 11 $C9C4 and bank 11 $C286.
   - **Bank 2 $DAC9-$DACC.** These 4 bytes are identical to the PLA/TAY/PLA/RTS epilogue used
     by four classified bank-2 routines: $CE9F, $DF08 and $DF81 (Observed) and $D307 (Static).
     The same bank already has accepted Static code that is entered with a stack contract
     (BenchRow_QueueField at $D2F4).
   - **No reference found.** No word (aligned or not) and no #hi/#lo immediate pair forms the
     address or address-1. No indexed or absolute operand covers these bytes, and there is no
     JMP (ind) or PHA/PHA/RTS path that reaches them.
2. **L-017: verified with exact bounds (retype only, 0 bytes).**
   - **ForceHandlers**, bank 15 $F92A-$F931 (phys 260410..260418, region 2081). The only
     caller, JudgePlayAtBase, uses Y = 0, 4, 8 or 12 (CPY #$10), and TYA/LSR turns that into
     4 word indexes.
   - **TagUpHandlers**, $F947-$F94E (phys 260439..260447, region 2082). It uses the same Y
     values. All 8 targets are instruction starts.
   - **CpuSideModeJumpTable**, bank 11 $D08E-$D18D (phys 184478..184734, region 1111). The
     lowest target equals the table end, and 128 words is the full range of the ASL dispatcher.
   - The region bounds are already exact. The L-015 code regions 2199 and 2200 and the bank-11
     regions 1928, 1930 and 1931 are referenced from these tables, so they become statically
     reachable.
3. **New L-019: bank 3 ScreenLoaderFrame inline jump table at $C048-$C04D (phys 49240..49246).**
   It is currently the first 6 bytes of generic data region 517.
   - The survey covered every JSR $849F/$9AA2 byte pattern in the ROM. Only this site and
     $D08B are followed by bytes not yet typed as jump tables.
   - Its words point to the Observed routines Init, Stream and Finish.
   - The extent rests on two facts: ScreenLoadStep only takes the values 0, 1 and 2, and the
     next byte, $C04E, starts a consumer-anchored table.
4. **L-011: the biorhythm effect is traced statically.**
   - Record bytes +$0C to +$0E hold the physical, emotional and intellectual values.
   - 14 in-play call sites read them through ReadTeamRosterByte or ReadSideTableByte, or
     through the $89E7 pointer table. The full list is in findings.json.
   - In most of these sites the value sets how much randomness is applied to pitch speed and
     curve, batted-ball speed and direction, CPU batter decisions and one walk message.
   - Every site except bank 11 $CEA3 and bank 15 $C944 is already Observed code.
   - Runtime value-flow tests X1-X3 were inconclusive: the snapshot window started after the
     pitch. They were not repeated because no fact remains for them to discriminate.
5. **L-018 and L-005: static checks only.**
   - L-018: no references to the debug or editor entry points were found.
   - L-005: bank 14 never calls FarCallBank. NMI-time far calls are not excluded.

## Ready leads for disassembly (recipes in analysis/leads.json)
- **L-016:** annotate 7 ranges as Code (Static, confidence 0.5, "no reference; unreachable as
  far as known"), reviewing each range on its own:
  - phys 38414+1, 39641+4, 100926+1, 180886+1, 182740+1, 248078+1, 252420+1.
  - Gain: 10 bytes. If all are accepted, the whole ROM is classified.
- **L-017:** in-place renames and evidence updates for regions 2081, 2082 and 1111, plus
  reachability notes on 2199, 2200, 1928, 1930 and 1931.
- **L-019:** split region 517 into a 6-byte jump table and a 154-byte generic data remainder.
- **L-011 (commentary only):** rename the record +$0C-$0E uses to biorhythm, for example the
  "rating" note at $91D5.

## Deferred or rejected, with reasons
- **Runtime fetch of force or tag-up plays (L-017):** this would only change provenance. It
  needs runners on base in a multi-at-bat sequence, and TK-004 makes that costly. The static
  table proof does not depend on it.
- **L-018 artificial execution:** it would only confirm straight-line mechanics, and no natural
  route exists to test.
- **L-009 and L-004:** semantics only, with no classification impact.
- **L-005 NMI-time far call:** low prospect.
- **X1/X2/X3:** X1 retained no events. X2 failed in a parsing bug. X3 ran outside the pitch
  window. All three are recorded in runtime.json and should not be replayed unchanged.

## Rejection review (in case disassembly rejects L-016)
- **Claim.** The seven ranges are instructions that were assembled as code and are
  unreachable. Reachability is not claimed.
- **Decisive prior evidence.** The round-2 disassembly said "unreferenced return opcode, no
  matching prologue". That rules out reachability and a known caller. It does not show the
  bytes are data.
- **Constraints combined this pass.** Gap census, byte value against fill, position inside
  routines, and the exact same-bank epilogue match. These are independent of each other.
- **Strongest competing reading.** Each site is an unread data byte or leftover data that
  happens to equal $60 or the epilogue.
  - It has no precedent in this ROM: every other embedded non-code gap is read by code.
  - It needs seven coincidences with the RTS value.
- **What remains insufficient.** No original source or toolchain artifact can prove that the
  author typed `rts` rather than `.byte $60`. The proposal therefore stays at confidence 0.5.
- **If rejected.** The bytes stay unknown. The next step is the reviewer's specific failing
  constraint. Another search for references is not useful, since the searches here were
  exhaustive for words, immediates and indirect jumps.

## Recommendation
Run another disassembly cycle: L-016, L-017 and L-019 are ready, and L-011 is ready for
commentary. After that, run commentary and redirect again. I found no other promising
classification avenue. Toolkit: no new toolkit issues found (TK-001 to TK-005 unchanged). X2
was a bug in my script, not in the toolkit. Disk: 4.0 GB free; scratch is 4.2 MB, and only
small x2/x3 outputs were added.
