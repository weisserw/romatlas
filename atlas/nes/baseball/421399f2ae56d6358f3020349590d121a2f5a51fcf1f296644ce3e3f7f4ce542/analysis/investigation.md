# Investigation ledger

## Round 1 disassembly (2026-11-18) — checkpoint pass-001

Game: Baseball (Nintendo, 1984), NROM-128. PRG file 0x0010-0x400F mirrored at CPU
$8000/$C000; all code views use CPU $C000-$FFFF with prg_banks [0,1,0,1] (static NROM
mirror, matches emulator context). Vectors: NMI $C046, RESET $C40B, IRQ/BRK $C43E (RTI).

### Structural findings (evidence for classification)
- INL-1 Inline-argument helper E99A: pops its own return and the caller-of-caller return
  into $00/$01; E9B4 loads inline bytes 1,2 into X,A with Y=3 and resumes the routine;
  the routine's final RTS lands in E9A7, which adds the routine's exit Y to $00 and
  `jmp ($0000)`. Inline length = exit Y - 1. Routines and lengths: C8BB 3 (ldy #4),
  C963 6, C985 6, C9A7 5, EB0A 3 (ldy #4), E29B/E29D 3 (exit via EB0D, ldy #4),
  E9DB 2 (Y=3 path; all 5 call sites have nonzero hi byte), EA63 2 (ldy #3).
- INL-2 EA17 own PLA mechanism: inline count byte + count bytes copied to $0300 buffer,
  rebuilt return = site+4+count.
- Contract verified at runtime: at 40+ call sites the continuation (site+3+len) was an
  observed opcode fetch and no inline byte was ever fetched as an opcode.
  84 call sites, 314 inline bytes annotated as Data (traces/r1-inline-sites.json).
- TASK-1 Cooperative task scheduler: EB0A/E29B create task (slot, entry word-1 stored in
  069F/06CF); EB94 resets SP and dispatches with PHA/PHA/RTS; EB41/EB48/EB4C yield/sleep
  (save caller return as resume address). Task entry words (+1) used as static roots
  (45 EB0A + 3 E29B sites).
- Only two return-address manipulation mechanisms exist (byte search for PLA-store
  patterns: E99A, EA1A); no other jump tables found (PHA/RTS only at EA3C, EB9B; JMP
  indirect only E9B1 and EEDC-area data).
- Static traversal (analysis/tools-r1/trav.py) from vectors + observed coverage +
  inline continuations + task entries; excludes dead fall-throughs after complementary
  branches, immediate-load+branch and CLC/BCC, SEC/BCS; rejects chains ending in
  unofficial/BRK/KIL opcodes (CD22, D339-D33B rejected, now data). Cross-checked against
  toolkit analysis.discover_context (identical set, 5146 instructions). No accepted
  instruction uses an unofficial opcode.
- Data formats proven by exact chain parsing: C8BB sprite lists [n][n x 4 OAM bytes]..00
  (EBD6-F4F1 contiguous, five lists); EA63 PPU string lists [hi][lo][ctrl][data]..00
  (C000-C023, F4F2-F88C, F88D-F94C); F94D 88-byte OAM init table; FA00 APU period pairs;
  FE40 sound data. FF fills: F9A5-F9CF, F9F4-F9FF, FA5A-FA5F, FFE2-FFF9 (Padding).
- RAM facts (bounded experiments): $33 half-inning counter (0..17; C6D3 cmp #$12 -> game
  end at C708), $0F/$10 controller roles for batting/fielding (C87E), $64 balls,
  $62/$63 presumed strikes/outs (written, game continued normally to GAME SET).

### Lead outcomes this pass
No pre-existing leads. Produced L-001..L-007 (analysis/leads.json).

### Remaining unknown (443 bytes, 17 ranges) — see leads
C959(10) D810(75) DBF6(59) DC91(41) DFEB(76) E2A5(46) E4B8(10) E5F4(1) E741(5)
E7BC(15) E822(6) E8B7(35) E9C2(25) E9FC(9) EB3C(5) EBB4(8) FAAE(17).
Checked: raw JSR/JMP/word/word-1 references to every byte of each range (none from
accepted code); all are official-opcode fragments or mixed code/table slices lacking an
entry. Kept unknown because code-vs-data entry evidence is missing (not because meaning
is unknown).

## Round 2 disassembly (2026-11-18) — checkpoint pass-002

### Lead outcomes this pass
- L-008 verified: WritePpuInline $EA05 = STX $05; JSR $E99A (via $E9B4: X=count at ret+1,
  A=first data byte, Y=3); STA PPUDATA loop X times; DEY; RTS to $E9A7 which resumes at
  ret+Y => inline length 1+count. Only JSR $EA05 site: $EAF5. Count 6 => EAF8-EAFE inline
  Data, EAFF RTS. Region 0x2AEE+44 split; EA05 added to workspace traversal inline contracts.
- L-001 verified (DC91-DCAB code, DCAC-DCAE E29B inline args -> entry $DC97, DCAF-DCB9 code,
  E4B8-E4C1 code with always-taken BNE into accepted tail $E4B5).
- L-002 verified (D810-D836 code; D837/D843/D84F three 12-byte tables, each last>=first entry).
- L-003 verified (DBF6/DBFE 8-byte tables after RTS $DBF5; DC06-DC28 code; DC29-DC30 offset pairs).
- L-004 verified (DFEB-DFEE after RTS $DFEA; DFEF-E01C stub chain + body; E01D-E036 one data block).
- L-005 verified (11 fragments; C963 inline length 6 re-derived: Y exits 7 -> resume $C962).
- L-009 ($E5F4) not reopened: no new evidence.
Structural evidence is static; reachability of all new code remains unknown (no caller found,
not observed). Do not describe these routines as dead/unused.

### Remaining unknown (1 byte)
0x2604 ($E5F4) — see L-009 rejection review in analysis/handoffs/round-1-redirect.md.

## Round 3 disassembly (2026-11-18) — checkpoint pass-003

### Lead outcomes this pass
- L-011 verified/applied (metadata only): region 648 (physical 0x1FA2-0x1FD3, $DF92-$DFC3,
  prg_banks [0,1,0,1]) keeps kind Code, provenance Static, same offset/length/views/confidence.
  Evidence replaced: structural = sum-of-squares tail X^2+Y^2 -> A:Y ($DF92-$DFA6) + SquareByte
  $DFA7 (shift-add, RTS $DFC3), exact official tiling, bounded by always-taken BEQ $DF90 and Data
  $DFC4; reachability = no effective entry found (BEQ $DF90 always taken; no outside reference,
  analysis/redirect/r2-a/sq2_refs.out; never executed in r1 coverage). The incorrect round-1
  "reached by bounded traversal" claim is removed. Not claimed dead/unused.
- L-009 ($E5F4) not reopened: no new evidence (round-2 redirect rejection review assessed sound).
- L-012 untouched (commentary only).
- No gameplay run: no remaining lead that gameplay could serve (scenarios in analysis/scenarios.json).

### Remaining unknown (1 byte)
0x2604 ($E5F4) — L-009, terminal residual per analysis/handoffs/round-2-redirect.md.

## 2026-11-18 — User-directed fallback for the residual byte
- 0x2604 ($E5F4) annotated as generic Data, Manual provenance, meaning unknown, by explicit user
  authorization (batch exception, <= 10 residual bytes). NOT an evidence-backed role; L-009's
  code-vs-data ambiguity is unchanged and remains documented. Record: analysis/handoffs/final-fallback.md.
- Remaining unclassified bytes: 0. Evidence-backed: 24591/24592; fallback: 1 byte.
