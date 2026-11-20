# r04-a1 redirect notes (round 4, input checkpoint pass-004)

Research only. Map, state, source, ROM unchanged. Input: 5 unknown bytes (L-027, lone $60).

## Retry record (required before a substantive retry)
- Target claim: each lone $60 (b0 $8D92, b4 $B979, b5 $ACFA, b6 $9745, b6 $EFE5) is a dead `rts`
  (Code, Static, reachability none known) rather than an unread data byte.
- Prior barrier/method/outcome: r02-a1/r03-a1 weighed each byte alone (terminal predecessor,
  classified successor, no reference, value $60) -> non-discriminating, blocked.
- Changed inference (coordinator Q-R4-1): combine the per-byte facts with ROM-wide statistics:
  orphan bytes after terminals, the padding convention, an authoring habit, neighbour extents.
- Expected discriminating result: if orphan bytes in code are systematically single $60s and no
  other padding exists, the combination might favour "dead rts". If other padding/remnant values
  exist or the $60 pattern has no analogue, it does not.
- Stop condition: survey done once; no runtime work (no caller/consumer exists to exercise).

## Method (static, read-only; scripts in this directory)
- survey.py: parses commentary/source/game.asm (pass-004 regions, .org, labels) and decodes every
  Code region from the ROM bytes (27,609 instructions; 4,237 labelled addresses). Terminal =
  rts/rti/jmp/jmp(ind), complementary branch pair (beq/bne etc.), branch after a flag-fixing
  lda/ldx/ldy #imm, clc/bcc, sec/bcs. An "orphan run" = bytes after a terminal up to the next label,
  static branch/jsr/jmp target or region boundary. Absolute-operand references into the run (base
  up to 16 below) recorded. Output: survey.json, survey.txt; detail.py -> detail.txt.
- denominators.py -> denominators.txt: counts of all terminals and what follows them.
- twins.py -> twins.txt: ROM search for copies of the bytes before/after each $60.
- Region scan for uniform $00/$FF/$EA Data runs (inline in the round-4 session; results below).

## (a) Orphan bytes after terminals
- 1,834 terminals (not at a bank end). 1,541 are followed directly by a label/static target.
  293 are not (survey.txt). They are, by kind:
  - referenced tables (absolute reads into the run, e.g. b0 $95D9, $CC54, b5 $ACDA, $FF2C);
  - entries of $8003+3n JMP vector tables and other pointer-reached code (Static/Observed code,
    unlabelled in source);
  - L-026 tier-B dead fragments classified Code because each has internal structure beyond the
    opcode: `jmp RemoveObj ($EE43)` x5 (b0 $CBA9, $DC81, $E07F, $E867, $EB53; operand = known entry),
    `sta ObjFrame,x / rts` (b2 $EFA9; 8 identical classified tails), multi-instruction fragments
    (b0 $8244, $AB91, $F097 ...);
  - $FF fill (below) and non-instruction remnants: b5 $8093 lone $FF (Data 0.8: ISC abs,X would
    overlap the code at $8094), b6 $8C13 `8B 8E` (Data 0.8: no instruction can end at $8C15),
    b6 $FE2C `$8D` (Data 0.8, truncated opcode);
  - the 5 unknown $60 bytes.
- $60 directly after a terminal occurs 56 times: 48 are shared exit rts reached by a branch
  (label/target), 3 are the first byte of a Data table (b0 $A86E, b2 $FC09, b5 $FF2C: $60 is also a
  data value here), 5 are the unknowns. No classified unreferenced lone rts exists anywhere.
- So orphan bytes are NOT systematically single $60: lone unreferenced orphans also have values
  $FF and $8B $8E and $8D; the $60 cases are 5 of 1,834 terminals.

## (b) Inter-routine padding of other values: yes
Uniform $FF Data regions between code (Static, conf 1.0 unless noted): b0 $8030 (3), b0 $FEC3 (23),
b0 $FF0E (10), b0 $FF6C (2), b1 $FF5D (27), b2 $802A (6), b2 $F131 (15, mid-bank after rts $F130),
b2 $FF6A (14), b3 $8054 (12), b3 $FF2E (64), b4 $FF73 (5), b5 $8093 (1, conf 0.8), b5 $FF32 (30),
b5 $FF6C (12), b6 $801E (18), b6 $FF50 (40), b7 $FF73 (5); plus b4 $B8BD (1, after data).
$00 runs occur only between data regions; no $EA fill. The codebase's padding convention is $FF,
including a single-byte case between an rts and code.

## (c) Authoring-habit analogues
- None for a dead rts. Every other terminal (1,829) lacks an unreferenced trailing $60.
- b6 $9745: the same exit `stx $0682 / jmp LoaderCommonTail ($E59A)` occurs 8 more times in bank 6
  (b6 $9432, $9510, $9809, $E6D3, $F207, $F78C, $F7BC, $F80D); none is followed by $60 (twins.txt).
- b6 $EFE5: the twin BgObj19_ChaseLeaderB ($EFEF) has the identical `lda #hi / bne L_B6_F00C`
  followed directly by its script $F003, with no $60.
- No ROM copy of the bytes around any of the five (twins.txt: pre 3-10 bytes, post 4-6 bytes).
- The accepted dead-code analogues (L-026 tier B, L-028 $FE24) each rested on structure a lone
  opcode lacks: a valid operand to a known entry, a repeated multi-instruction idiom.

## (d) Neighbour extent proofs
- b0 $8D92: code on both sides (branch pair; Observed CheckYDistance). No adjacent table to join.
- b4 $B979: RamRoutineTemplate_B4 copy loop `lda $B97A,y`, Y 0..$0C: template is $B97A-$B986. Excluded.
- b5 $ACFA: code on both sides (GetTerrainHeight `jmp $AFB9`; SnapToTerrain). Table $ACDA-$ACE0 is not adjacent.
- b6 $9745: StageNextTables read with Y = StageId >= 7 (`cpy #$07 / bcc` guard at $9723), bases
  $973F/$9744 -> $9746-$974F; no index-6 slot (r03-a1). Excluded as a read element.
- b6 $EFE5: script pointer immediate $EFE6 (`lda #$E6 / lda #$EF`); StepBgObjScript reads
  (ptr),y with Y = $0716 offset forward from 0. Excluded as a script byte.
These proofs exclude membership in a known neighbouring structure (already known in r03) but
not an isolated unread data/remnant byte, which is the competing interpretation.

## Assessment (evidence-policy.md)
The coordinator's combined premise fails on the evidence: (1) a padding/fill convention exists
and it is $FF, including a lone $FF between an rts and code (b5 $8093); (2) other unreferenced
non-instruction remnants exist ($8B$8E, $8D), so isolated stale bytes occur in this ROM;
(3) there is no dead-rts authoring habit: 0 of 1,829 other terminals and 0 of 9 local twins
(8 identical LoaderCommonTail exits, BgObj19) carry one. "Uniform $60 pattern" is the one fact
already used (five bytes share a value), not an independent constraint.
The $FF convention does weaken "conventional padding" as the rival for a $60, but the rival
that remains is an unread stale/remnant byte, which (2) shows is a real class here. A stale
byte could be an old rts (code-origin) or an old data value ($60 starts two referenced tables);
the evidence cannot tell which. Per policy an unreferenced return opcode lacks structure on its
own; the combination adds no constraint that discriminates. Keep unknown.

## Other avenues
None actionable. All other bytes are classified; commentary readability complete; gameplay
checklist visited (scenarios.json). A future reference/consumer for one of the five would
reopen it. No server was started.
