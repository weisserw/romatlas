# Handoff: round 1, phase 3 (redirect) - attempt r1-redirect-a

**Recommendation: another disassembly cycle** with ready leads L-012, L-013, L-014, L-015,
L-010 (plus the L-003 progression recipes for runtime confirmation). Not terminal; no
rejection review required.

## Scope
Static redirect over all 239 unknown ranges (17,912 bytes) of the input checkpoint, the 14
unproven JSR $849F jump tables (L-002/L-007/L-008), the $EFF0 boundary (L-010), and the
progression RAM contract (L-003) with state-patch shortcuts to batting, game end, season end,
post-season and the ending. Research files: analysis/redirect/r1-redirect-a/ (NOTES.md frontier,
tools/, tables.json, walk2-c.json, proposals.json, unknown-ledger.json, rescat-c2.json,
cc-dis.txt, runtime.json). No map, state, source, ROM or binary changes.

## Checkpoint / verification
- Input checkpoint: maps/round1-disassembly-b.json (sha256 2b95d9dd...); 375,320 / 393,232
  classified (95.44%), 17,912 unknown. analysis/state.json untouched. input/game.nes unchanged.
- Proposed (not accepted): 17,892 of the 17,912 unknown bytes have a disposition in
  proposals.json; 20 bytes stay unknown. Accepted bytes this phase: 0 (redirect does not annotate).
- Static walk check: tools/walk2.py (round-1 static_walk engine + proposed tables) adds 6,112
  code bytes, 0 clashes with classified regions, 0 invalid decodes, 0 overlap conflicts.
- Runtime check (artificial state, traced): every observed instruction start that falls in the
  proposed L-012 code (union 372 over T1-T3: bank 2 331, bank 15 41 incl. mercy handler $D31B) equals a walk instruction start; none fell in
  data proposals (runtime.json).

## Findings
1. Jump-table extents (L-012). 20 tables proven by independent constraints: lowest in-range
   target = table end with all earlier words valid (NMI task table bank0 $86F4 x10, PlayState
   table bank15 $C048 x128 -> default handler $C148, bank4 x5, bank2 x5 found in new code);
   selector clamps (bank11 $C282 x2, bank15 $D386 x2, bank2 $E39C x4, bank2 $E376 x3 via
   byte table $E37C); bank2 $D157 x9 (next word is RAM, $D169 is a branch target); bank2
   FrontEndScreen table $C177 x88 (followed by byte data); bank11 CPU-pad tables $C49B/$D08E
   x128 (lowest target = start+256). Walking the handlers yields 6,112 code bytes: bank 2
   front-end screens ($DFFF-$E697, $C25F.., $D72F..), bank 11 CPU pad states ($C8D0-$CD91,
   $D1AA-$D319), bank 15 PlayState handlers ($CC08-$CEEB, $CFEB, $D31B mercy rule), bank 4
   script handlers, bank 0 NMI tasks 1/4/6/8 ($82AF nametable clear, $8708, $8305, $877C).
2. Consumer-anchored inline tables (L-013): 106 ranges, 2,024 bytes read by lda/cmp/adc/sbc
   abs,x/y with no control reference and no fall-through (incl. ASCII team-name tables).
3. Pattern data (L-014): 81 ranges, 7,744 bytes inside data blocks: DMC sample spans
   (bank 1 tables $C0F1/$C0F8 = 7 entries bounded by cmp #$07; bank 10 for VoiceSample 7..10),
   player records with ASCII names (bank 2 $E955-$FCC6), pointer lists/metasprite pieces.
4. Unreferenced self-consistent routines (L-015): 36 islands, 1,372 bytes (math helpers in
   bank 0, binary->decimal bank 5 $ECD8, debug end-game shortcut bank 15 $C606, StartSeasonDay/
   SeasonWins d-pad editors bank 2 $E6CC/$E73D, map-block helpers bank 8/9). Reachability
   unresolved; lower confidence.
5. L-010 confirmed: $EFEC BCS / $EFEE BCC is exhaustive, so the 1-byte "RTS" at $EFF0 is a
   walker artifact; the table is 4 bases x 4 bytes $EFF0-$EFFF. tools/exhaustive.py finds no
   other static-only artifact of this kind.
6. Progression (L-003, runtime.json, analysis/gameplay.md): contract confirmed (OutCount $0356,
   HalfInning $0066, totals $05E6/$05F3, SeasonDay $0610, wins $0616, human side $0614 bit0).
   Visited with patches: batting, top-9th game end (PlayState $70, wins +1, NEXT PASSWORD
   screen, SeasonDay +1, biorhythm screens), mercy rule "CALLED GAME", post-season series
   (SeasonDay $82+), series clinch -> FrontEndScreen $44 ending/credits pictures.

## Ready leads (analysis/leads.json)
- L-012 (high): tables.json + walk2 recipe; proposals.json groups L-012-tables/-code/-selector-bytes.
- L-013 (high): proposals.json group L-013-anchored-data; evidence rescat-c.txt/gapref-c.txt.
- L-014 (medium): group L-014-pattern-data; evidence rescat-c2.json features.
- L-015 (medium/low): group L-015-code-islands; listing cc-dis.txt; reviewer may reject islands.
- L-010 (high, small): replace Code 0x3F000 + unknown 0x3F001-0x3F00F with one Data table.
- L-002/L-007/L-008/L-001: recipes executed and packaged in L-012..L-015 (status ready, history appended).
Recipe for all: run `python3 analysis/redirect/r1-redirect-a/tools/walk2.py
analysis/redirect/r1-redirect-a/tables.json analysis/redirect/r1-redirect-a/roots0.json out.json`,
then annotate per proposals.json with prg_banks given per range (bank 0: [0,1,null,null];
bank b: [0,1,2b,2b+1]); re-run tools/residual.py to confirm the 20-byte residual.

## Rejected / deferred avenues
- 20 residual bytes (orphan RTS bytes bank2 $D5FE, bank6 $CA2E, bank11 $C286/$C9C4, bank15
  $C8FE/$D9F4; bank0 $8420; bank4 $D174-$D17B; bank6 $C845-$C849): no reference, too short for
  a contract argument; deferred until a caller appears.
- L-004 (semantic roles of large data blocks), L-005 (bank-14 ID byte): no classification impact;
  deferred (diminished prospect for byte gain).
- L-009/L-011 (voice-sample and biorhythm effects in play): biorhythm screens and voice requests
  observed in passing; effect on batting/pitching not traced - needs a traced at-bat with record
  reads (TK-004 makes long traces costly). Deferred as commentary semantics, no byte impact.
- Long traced batting segment (45 taps) killed at the 30-minute limit (TK-004 addendum);
  replaced by untraced RAM logging plus short traced segments.

## Unresolved questions
Natural callers of the L-015 islands; whether the bank 15 $C606 debug shortcut is reachable
(controller-2 bits in $55); password entry (CONTINUE) flow not exercised; exact meaning of
FrontEndScreen $40/$44/$56/$57 handlers (now in L-012 code).

## Toolkit
TK-004 addendum (trace slowdown; not blocking). No new issues; see analysis/toolkit-issues.md.
