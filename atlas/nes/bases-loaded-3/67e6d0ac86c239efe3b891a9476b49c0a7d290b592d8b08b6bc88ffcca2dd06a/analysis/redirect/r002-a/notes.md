# Redirect r002-a (round 2, input checkpoint pass-002)

Scope: the 3 unknown bytes (L-015), commentary open question 4 (L-016), spot-check of L-013 data.
All work static on pass-002 (maps/current.json); no emulator/serve used; ROM/map/source unchanged.

## Byte context (exact)
- 23:$B402 (phys 193554, ctx [22,23,30,31]): `... cmp #$3c / bcs SetLevel1 / bcc SetLevel0` (90 01, always
  taken after the bcs) | 60 | SetLevel0 `a9 00 f0 02`.
- 28:$8620 (phys 230960, ctx [28,29,30,31]): CpuPitchTiming @skip2 `... bcc CpuPitchTarget_Return /
  sta CpuPitchDelay / rts` | 60 | CpuPitchTarget `a2 00 20 81 86`.
- 28:$9883 (phys 235667): PitchPhaseTest `lda PitchFlags / and #4 / bne CallStrike (d0 02) / rts` | 60 |
  CallStrike. The bne skips both RTS bytes.

## Checks (evidence/*.txt, tools/*.py)
1. rts_refscan.txt (scan.py, refs2.py): branch/JMP/JSR targets of all 24,316 classified instructions,
   abs/abs-indexed reads spanning the targets, data words equal to target or target-1, immediate
   lo/hi pairs. Only hits: LDA $B30B,x/$B30C,x (ScoreLevelCursor, X in {0,2} -> never $B402);
   coincidental words in field data banks 2/3/4 and bank 18 (no consumer maps them with bank 23/28);
   3 LDA #$02..LDA #$B4 pairs in banks 26/27 that store to $19/$18 (= RAM $02B4, not a pointer).
2. rtstrick.txt: no straight-line PHA,PHA..RTS/RTI dispatch exists in classified code. JumpTableEngine
   ($F2xx) and FarCall use JMP (ind) with direct addresses, so no target-1 encoding is in use.
3. Copies: 6/8/10-byte sequences before/after each byte occur nowhere else (no bank variant like 0/4 vs 2
   or 6 vs 16 to compare).
4. Coverage: none of the bytes is an observed instruction start in any recorded scenario (5,380 starts).
5. rts_idiom_census.txt (rtsrts.py, zero.py): 48 terminator+RTS/JMP adjacencies in classified code; all
   that lack a branch reference have an inline jump-table word reference (e.g. 28:$9163/$9164 RTS stubs),
   i.e. no precedent of an unreferenced lone RTS classified as code.

## Decision: genuine ambiguity (keep unknown) — see rejection review in round-002-redirect.md.

## L-016 (semantic, static)
BatContact @skip3 (28:$9670) is the only writer of batter state $20 (group 2): not swinging, RngState&$70=0,
ball X hi < $68, ContactDepth set. BatterStep2_Start: normal -> Hold -> PitchOutcome 1; late (HalfInning>=10),
close (Visitor-Home in -4..3), Var_0356>=10, LoopCounter&$70=0 -> Pose -> Leave -> PitchOutcome 2 ->
team banner. Not runtime-confirmed.

## L-013 spot-check (data_codesig_spotcheck.txt, datachk.py)
Scanned all PRG Data for JSR/JMP to fixed-bank code entries and LDA/STA to PPU/APU/MMC3 registers: 11
hits, longest clean official-opcode run ending at a hit = 6 instructions; bank 1/3 hits recur every $63
bytes inside field data (periodic records). No conflicting code evidence.
