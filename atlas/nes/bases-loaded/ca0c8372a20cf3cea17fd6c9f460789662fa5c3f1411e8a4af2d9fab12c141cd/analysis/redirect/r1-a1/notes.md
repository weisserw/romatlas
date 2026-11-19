# Redirect r1-a1 research notes (round 1, phase 3) — working log

Input checkpoint: pass-001 (byte_identical true), unknown 14,557 bytes / 185 ranges
(analysis/redirect/r1-a1/unknown-ledger-input.json from unknowns.py).
Tools: probe.py (static closure probe over scratch/static.py; reports unknown bytes covered,
conflicts); farcalls.py (FarCall site enumeration -> farcalls.json); exp.py (runtime helper).

## F1 FarCall fall-through (methodological correction, global)
Prior static traversal (scratch/static.py switches_prg) treated any JSR whose closure reaches
FarCall ($817D) as a bank switch and stopped fall-through. FarCall (bank 0 $817D, identical
mechanism in bank 3 with $0576/$0577/$0557) pushes the caller's selection, calls
(FarCallVector) with return $819D and FarCallReturn restores the caller's banks before RTS.
So JSR FarCall / JSR wrapper-that-ends-in-JMP-FarCall returns in caller context.
probe.py with this rule from all 1,056 existing Code region starts: +3,392 unknown bytes
covered; flags (invalid/barrier/overlap) are identical with and without the rule (8 invalid +
3 barrier, all pre-existing paths: JSR $FFFF placeholders, bank-1 $9E2E skip trick, $8ECC, $F655).

## F2 AtBatStateTable bounded (L-011)
RunAtBatState: LDA AtBatState / ASL / TAX / LDA $B199,X / LDA $B19A,X / JMP (Ptr00).
Table $B199-$B298 (256 bytes, 128 words, phys $031A9-$032A8) ends exactly at the first handler
$B299 (AtBatNewBatter). Every word is either $B198 (the 1-byte RTS at phys $031A8, unknown) or a
handler; used states $00-$02,$10-$18,$20-$28,$2A,$30-$33,$40,$41,$60-$62. New entries: $18->$B9B6,
$17->$BB86, $22-$24->$BC66, $2A->$BD9A, $60->$BEDE, $61->$BEF5, $62->$C000 (change-sides handler:
waits SixFrameTicks>=$1E, INC HalfInning, end-of-game test at HalfInning >= $18 / >= $11 with
VisitorScore $0418 vs HomeScore $0426, then RequestScene $0F via $F6B1).
Probe with these roots + F1: +714..1027 bytes in bank 0 without conflicts.

## F3 Bank 2 $9551 gap (L-002 correction)
Bank 2 vector $FF06 = JMP $9551, reached via bank-3 wrapper $84D1 (FarCall sel $20) called from
bank 3 $88EB and $8A0E. $9551-$95A2 is code (copies 14x10 tiles from $9DE3 to PPU $2109 and 64
attribute bytes from $95A3 to $23C0). $9511/$951C (vectors $FF00/$FF03) copy 1024 bytes from $95E3 /
$99E3 to both nametables. Split: code $9551-$95A2 (82), attr $95A3-$95E2 (64), nametable image
$95E3-$99E2 (1024), nametable image $99E3-$9DE2 (1024), tile block $9DE3-$9E6E (140),
$9E6F-$9EAB (61) code-like, stack-balanced, exits only into PlayerDataAddr $9EAC (JMP $9EAC and
fall-through), no reference found (no JSR/JMP/immediate pair) -> separate candidate.

## F4 Bank 1 $F074 (vector $FF00) and $E8EF data
Bank 1 $FF00 = JMP $F074, reached via bank-3 wrapper $84A8 (sel $10) called from bank 3 $86A0.
$F074-$F0ED code (122 bytes; random choice of 1-3 distinct values into $056A-$056C), $F0EE-$F0F0
3-byte table read by LDA $F0EE,Y (Y<3), $F0F1 = $FF.
Bank 1 $E8EF-$EEAE (1472) is data: 8-byte records; CpuPitch handlers $E828/$E837/$E84F/$E871/$E895/
$E8AF (jump table $E81C) load base pointers $E8EF, $E96F, $EA6F, $EBEF, $ED6F then
index = (situation 0..15 from $EEAF) * k + count adjustment, *8 at $E8BE; table extents 128/256/384/384
and 256 for $ED6F; tail $EE6F-$EEAE (64 bytes) same record format, beyond computed index range.

## F5 Sound tables bank 2 $ABCE-$AD8D and banks 4-7 $A4AC-$A66B (448 each, identical)
Visibly non-instruction tables (16-byte rows: $FE/$01/$02 runs, descending/ascending envelope
nibbles, final 256-byte 16x16 volume scale $30|v). Consumers: bank 2 $AABF ORA $ABEE,Y and $AAC8
ORA $AC8E,Y; banks 4-7 $A39D ORA $A4CC,Y and $A3A6 ORA $A56C,Y.

## F6 Banks 4-7 $A000-$A022 (35 x 4)
Byte-identical to bank 0 $9E23-$9E45 (observed code with LDX-abs skip trick). Called by JSR $A000/
$A003/$A00B/$A01B from the classified (static) module copy in banks 4-7 ($80A9/$80AC/$80ED/$80F0).

Note on F4: bank 1 $F074 chain is NOT supported as reachable: wrapper $84A8 is only called from
$86A0 inside unreferenced InitGameRam $864D (no JSR/JMP/word ref). Moved to L-024; only the 4-byte
table $F0EE in L-023 is kept (its consumer is inside that chain; generic data either way).

## F7 Gameplay RAM contract and end-of-game/season experiments (artificial state)
- RAM: HalfInning $69 (inning*2+side, odd = bottom), OutCount $0771, StrikeCount $0772,
  $0418 = bottom-half/home total (AddRunsToScore table $C158: odd HalfInning -> $C15C adds to
  $0418/$0419,X) and $0426 = top-half/visitor total -> commentary labels VisitorScore/HomeScore
  are swapped (L-025). PennantCounterA $0766 = games played (NextScheduleEntry increments; game-end
  scene compares with $84 = 132 games), PennantCounterB $0767 = wins (scene 17 compares with $50).
- Developer shortcuts exist in bank 3 ($9DEE: pad2 A -> $0418=$0419=7, HalfInning=$10, strikes=outs=2;
  $9E47/$9E5B: pad2 sets $0550/$054F and copies to PennantCounterA/B) - unreferenced (L-024) but
  they confirm the patch contract used below.
- E1 (analysis/redirect/r1-a1/experiments/e1.py): game1.snap (pennant, top 1st), write $69=$10 $0771=2 $0426=5 $0418=0, press A
  per pitch: out -> AtBatState $62 -> HalfInning $11, game continued (bottom 9th, home trailing).
- E2: $0418=5/$0426=0, 40 pitches, no out (CPU hits; fielding not driven) - no result.
- E3 (analysis/redirect/r1-a1/experiments/e3.py): same patch + StrikeCount forced 2 before pitches; out after 85 cycles ->
  state $62 (snapshot scratch/snapshots/r1a1-e3-62.snap) -> HalfInning 0, scene 15 game-end screen
  "PENNANT RACE D.C. 2 - PHILLY 5, PHILLY 1 GAMES 1 WIN, PASSWORD EDCBAAK" (shots/e3-game-end-pennant-race.png).
- E4 (analysis/redirect/r1-a1/experiments/e4.py): from r1a1-e3-62.snap write $0766=$84 $0767=$55 -> scene 16 -> scene 17
  "PENNANT RACE 132 GAMES 86 WIN PHILLY" -> scene 12 (ending figures) ... (see S11 in scenarios.json).
- Coverage (trace aggregates, capacity 0) in analysis/redirect/r1-a1/obs-r1a1.json: 331 instruction
  starts fall in L-015 proposed unknown bytes; none in residual or classified data.

## F8 Small items
- L-021: NmiBusyFrame bank 0 $85C5..: JSR SetPrgBank(ChrBankSelect&$30) then $85D5 LDA $6D/AND #$43/
  JSR SetChrBank falls into observed @soundOnly $85DC; banks 1-3 have the same at $8386/$8389.
- L-022: table $B920 = [$B91F unused, $B928, $B931, $B93A], index ($0320,X & 3), 0 branches away.
- L-023 tables: see proposals.json notes. $AC8E metasprite ends with $80 at $ACFE.
- NmiModeTable $84F2: NmiMode values written are 0,2,4 (bank 0 $9AE9 =2, bank 1 $83E7 =0, bank 3
  writes 4/6 for its own NMI table); 3 entries end at code $84F8 -> no change.
- FieldTargetTable $87C1: 8 words, all targets classified; bounded by code at $87D1 -> no change.

## F9 Residual after all ready proposals (residual-after-all-proposals.json: 2,461 bytes, 50 ranges)
bank 0 355, bank 1 125, bank 2 61, bank 3 1,920. linear-audit-residual.json: almost all decode
as clean RTS/JMP-terminated routines with valid callees but no entry (no JSR/JMP/branch from an
instruction start, no immediate pointer pair, no LE word in data). Several are preceded by an RTS
that live callers target (disabled-debug pattern). Data parts referenced only by such code:
bank 3 $B73F.. tables inside $B65F+252, $CB40 records inside $CAFA+75.

## F7 (continued) Ending
- E4 result: scene 16 -> 17 ("PENNANT RACE 132 GAMES 86 WIN PHILLY", shots/e4-season-end-132-games.png)
  -> scene 12 requested but the screen never changed (ModeFlags=$C0, DpcmRequest $077B=0).
- E6 (analysis/redirect/r1-a1/experiments/e6.py, run_until): $077B becomes 0 five frames after scene 17 starts, PC $9FC7 bank 3 in
  the RAM clear $9FAF called from SaveGameState $9F12 (JSR at $9F5D) from SceneInit17 $8821. The
  preserved-byte list omits $077B; MainLoop then FarCalls bank (0&$B0)=0 $BD00 each iteration and
  only restarts the mode when DpcmRequest >= $F0 -> stall (lead L-026; natural behaviour unconfirmed).
- E7 (analysis/redirect/r1-a1/experiments/e7.py): from r1a1-e4-end.snap write $077B=$FF -> scene 12 ending runs (celebration
  figures, shots/e7-ending-scene12-*.png).
- obs-eval.json: over E1-E7, 352 observed instruction starts inside L-015 runs (20 runs), 0 inside the
  residual (L-024) and 0 inside classified non-code -> the end-of-game, season-end and ending scenes do
  not execute any residual routine.

## Frontier / continuation status
Investigation complete for this redirect pass (not interrupted). Deferred: L-024 (no entry), L-012
(password; not needed for progression), L-026 (behaviour), L-006/L-008/L-013/L-014 (labels only).
