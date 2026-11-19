# Redirect r2-a1 (round 2, phase 3) — research notes, 2026-11-19

Input: checkpoint pass-002 (byte_identical true), 1 unknown byte (phys $19CBC, bank 3 $9CAC).
Handoffs read: round-2-disassembly.md, round-2-commentary.md, round-1-redirect.md; leads L-012, L-026,
L-028, L-029, L-030. Prior attempt r1-a1 is referenced, not modified. Redirect changed only research
and lead records (no map/state/commentary/ROM/binary change). input/game.nes sha256 unchanged.

Unknown complement (exact, from analysis/state.json): 1 range, phys $19CBC+1, 32K bank 3 = 8K bank 12,
CPU $9CAC, context prg_banks [12,13,14,15]; neighbours: region 1335 Code/Static $19C40-$19CBB
(DrawSeasonRecord + FillNametableTile1, ends JMP $9C94), region 649 Code/Observed $19CBD+ (DrawBatterLine).
Disposition: proposed Code (L-028, below).

## F1 L-028 — the last unknown byte ($9CAC, value $60)
Claim needed: structural role code (one RTS instruction), exact bounds 1 byte, bank 3 context. No
reachability claim.

Checks (deadrts.py -> deadrts.json; l028_checks.py -> l028-checks.json):
- Bounds: preceded by JMP $9C94 (complete 3-byte instruction, unconditional barrier, $9CA9-$9CAB);
  followed by the observed entry of DrawBatterLine $9CAD (callers JSR at three sites in bank 3). The gap
  is exactly one byte; it cannot be part of either neighbour and admits no multi-byte table/record.
- References: no LE word $9CAC/$9CAB anywhere in PRG except inside DPCM_Bank7_Sample1 (bank 7 $D95B,
  1-bit audio, not a pointer); no LDA/LDX/LDY immediate pair $AC/$9C or $AB/$9C (FarCall/pointer setup)
  anywhere; no JSR/JMP/branch from any classified instruction start.
- Data consumers: the only absolute/indexed operand with base in $9BAD-$9CAC in bank 3 code is
  LDA $9C28,X (SceneSpritePalette, X<8 by CPX #$08) -> cannot reach $9CAC. Immediate $9B/$9C loads in
  bank 3 ($8E99, $B7D0, $B7FD, $B80E, $CC05) are PPU-address/coordinate values, not pointer highs.
- Idiom count: the ROM has 26 other $60 bytes that directly follow a terminating RTS/JMP/RTI and are
  directly followed by code; all are classified Code. 15 have direct JSR/JMP/branch references (e.g.
  null hooks $9E46/$9FD2 called by live code), 4 lie in statically traversed code (e.g. $CBAF
  NullSceneFrame, a scene-table target), and 7 are unreferenced L-024 bytes, incl. NullHookABDD (bank 0),
  NullHookBB35, NullHookC052, NullHookC38B (bank 3), accepted as Code in round 2 as the first instruction
  of L-024 gaps. Those accepted bytes have no more per-byte evidence than $9CAC;
  the following unreferenced routine made them part of a larger gap, but it does not evidence the RTS
  byte itself. Rejecting $9CAC while keeping those is an inconsistent criterion.
- Competing interpretation "1-byte data/fill": no consumer (above); this ROM's fill is $FF (padding
  regions) and the only <=3-byte non-code region between two code regions ROM-wide is $1888D (3 bytes,
  consumer-anchored LDA $887D,X), so an unanchored 1-byte inter-routine data byte has no precedent.
  The $60 bytes that start classified data ($BBA7, $C567 bank 0; $887A bank 1) begin long tables whose
  continuation is visibly data; here the next byte is a live code entry.
- Competing interpretation "dead tail of FillNametableTile1" (an unreachable RTS after JMP): also code.
Conclusion: code vs data is not materially ambiguous; propose Code/Static, label NullHook9CAC, confidence
~0.6, reachability unresolved (no entry). Exact proposal: proposals.json.

## F2 L-029 — boundary of AnimationStepTimes / DrawStepFigure (bank 3 $A398-$A3BB)
Bytes $A398: 40 F0 40 40 | 40 (then AD 79 05 = LDA SceneAnimStep). RTI as the first instruction of a
JSR-style routine ending in RTS is semantically impossible (pulls P and a return address without +1).
The sibling step-timer tables FigureStepTimesA ($A47C: 40 40 40 40 40) and FigureStepTimesB ($A4FD:
18 18 18 60 40), read by the same CMP table,X / INC step / CMP #limit code shape, are 5 bytes each and are
each followed by a routine start. So "timer routine; 5-byte table; routine" repeats 3 times. Consumer
AnimationStepTimer reads only X<2, so entries 2-4 are generic table bytes (no meaning claimed).
Proposal: Data $1A3A8+5, Code $1A3AD+31 (byte-identical; 0 unknown-byte change).

## F3 L-030 — ScaleLaunchByHomeLead (bank 1 $85BE): caller and actual effect
Static: the caller is known — bank 0 $9B6C sets FarCallVector=$85BE and FarCalls with A=$10 (bank 1)
after randomising BallVelZ, only when ModeFlags bit 6 (pennant/computer) and HalfInning even (top: the
visitors bat). The commentary's "caller not identified statically" is wrong.
Contract: the first FarCall ($9EFB DivU16Z, verified bytes A9 00 85 05 85 06 A0 0F) divides
(Ptr00+1=wins, Tmp02=0) by (Tmp03=0, Tmp04=games), i.e. wins / (games*256): the quotient is 0 whenever
games >= 1, so BNE fails and JMP $86A9 sets LaunchSpeed = $0400. Bank 0 $90B2 is MulU8 (bytes
A9 00 85 13 85 14 85 15 A0 07 46 12: MathA/MathAHi -> MathB), so the later products indeed read stale
Tmp03/Tmp04 — but that path needs games = 0 (division by zero).
Runtime X1 (artificial; experiments/x1_l030.py, x1_g10.json, x1_g0.json): game1.snap (pennant, top 1st,
CPU visitors batting), $0418=3, $0426=0, pitches with A. Entry at $85BE in banks [4,5,6,7] observed 3x
with games=10/wins=5 and 2x with the snapshot's own games=1/wins=0: path $85F5 -> $86A9 every time,
LaunchSpeed $01D7/$0266/$03BA -> $0400, LeadMargin = lead. PennantCounterA is already 1 during the first
game, so the games=0 (MulU8-mismatch) path is not reachable with natural pennant values observed.
Result: when the home side leads in a computer game, every visitor batted ball gets LaunchSpeed $0400
(the clamp value). Commentary correction only.

## F4 L-026 — season-end stall is the deterministic code path
Static: SceneInit17SeasonEnd ($880C) runs StartMusicI (InitSound -> DpcmRequest $077B=$FF) BEFORE
SaveGameState, whose ClearRamPages zeroes $077B (not in the preserved list). The other SaveGameState
callers either start music after the clear (game-result scene $8930: SaveGameState then StartMusicH/Loss;
SceneInitTitle: ResetGameState then StartTitleMusic) or leave by JMP to the reset (SceneFrame18). The main
loop performs a requested mode change only when DpcmRequest >= $F0 and otherwise FarCalls bank (0&$B0)=0
$BD00 (a fragment of AtBatHomeRunFinish: AdvanceBattingOrder, CountRunsScoredB, AddRunsToScore). No
writer of $077B lies between SceneInit17 and either exit request on the scene-17 path.
Runtime (artificial, from r1a1-e3-62.snap with $0766=$84):
- X3 (x3_l026_frames.py, x3_10.json): SceneFrame17 $A09F still runs each frame in scene 17; main loop
  reaches bank 0 $BD00.
- X2 (x2_l026.py): wins $10 (<$50, losing exit). Start held 4 or 40 frames: no effect (Start is not bit
  7). A held 6 frames: CurrentScene=9, ModeFlags=$C0 requested, DpcmRequest 0 -> stays stalled 600 frames
  (x2_lose.json, x2_lose_final.png). Control: same + write $077B=$FF after the press -> title runs and the
  attract demo (scene 20) follows (x2_lose_ff.json, x2_lose_ff_final.png).
- r1-a1 E4/E6/E7 already showed the winning exit (scene 12 ending) stall and its $FF control.
Conclusion: both season-end exits stall in natural play too (same code path; counters only select the
exit). Natural 132-game play not performed. Behaviour note for commentary (probable game bug); no bytes.

## F5 Commentary corrections found (button bit)
ram.inc defines Pad1Held bit 7 = A (ReadControllers shifts A in first). SceneFrame17 ($A09F, "wait ... or
Start"), SceneFrame18 ("Start restarts") and EndingStartToTitle ($C700, "Start on pad 1") test
Pad1NewPress AND #$80 = A button. X2 confirms: Start had no effect, A triggered the exit.

## F6 L-012 — password decode verified
X4 (x4_l012.py, x4_l012.json, artificial: letters written to SceneListA on pw.snap, 3 frames): EDCBAAK
(the natural password from r1-a1 E3) decodes to team 3, opponent 4, games digits 2,0,0 and wins 1,0,0
(units first; MultiplyAdd units/tens/hundreds) = PennantCounterA 2, PennantCounterB 1; SceneController 0
(valid); re-encoded letters equal the input. r1a1-e3-end.snap at the password screen holds exactly
TeamIdA=3, TeamIdB=4, A=2, B=1. EDCBAAA re-encodes with checksum K (=4+3+2+1 mod 16). Note:
PennantCounterA is 1 during game 1 and 2 at its end screen (game number rather than games played).

## Other avenues reviewed
- Round-1 heuristic generic-data rule / L-024 static code: deadrts survey found no contradicting control
  flow (the three $60-led heuristic data blocks are long tables; nothing enters them). Not reopened.
- L-005..L-008, L-013, L-014: label/semantic only, no structural question; unchanged.
- S12 DPCM capture: still attempted only; samples are classified; no structural value.

## Frontier
Not interrupted. After L-028/L-029 are applied, the unknown complement is empty; remaining work is
disassembly review of the two proposals, commentary corrections (F3-F6) and final editorial audit.
