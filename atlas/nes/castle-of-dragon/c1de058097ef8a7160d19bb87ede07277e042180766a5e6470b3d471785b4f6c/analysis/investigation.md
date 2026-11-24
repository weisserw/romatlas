# Investigation ledger

Follow workflow.md's completion and investigation contract.

## Round 1 disassembly (checkpoint pass-001b; pass-001 superseded, kept)

### Architecture findings
- UxROM: SetPrgBank $C50D (A=bank -> $06B1, STA $FFD0,Y bus-conflict table), SetPrgBank0 $C504.
- Cooperative multitasking: CreateTask $C200 (stack slots $0100+$20*n, entry pushed as RTS address), RunTasks $C17E, TaskYield $C1DC. Two task entries: $D002 (main/game) and $F712 (task 1).
- Code lives only in bank 7 (fixed) and bank 6 ($90D8-$BFE9 code with interleaved tables). Execution contexts observed: bank-6 code only with prg_banks 12,13 mapped; no instruction fetch in banks 0-5 in any scenario.
- LoadTileSet $C51B: pointer table $C87A (129 ptrs) -> 3-byte descriptors $C97C-$CAEC (bank in hi bits, count) -> CHR-RAM. Union of descriptor extents covers banks 0,1,3 fully and bank 2 except 16 zero bytes at $8AF0.
- Bank 4 $BC00-$BFFF palette records (LoadPalettes $C2D9). Banks 4/5 otherwise generic data read by fixed-bank loaders.
- Dispatch tables: ObjHandlerTbl_9632 (JMP ($0530) $962E), ObjHandlerTbl_A651 ($A63F), StageHandlerTbl_AC38 (by $052A), SoundCmdRtsTbl_CF3A (RTS dispatch $CC2C-$CC3E).
- Hidden test mode (input table $D0CA); $DCA2 contains unofficial `NOP #$04` (89 04) that is actually executed (T2 observed fetch).

### Method
- Toolkit static discovery stops at every JSR on mapper 2 (TI-001); used local recursive descent seeded by all observed opcode addresses + vectors + task entries + bounded tables (tools in analysis/evidence/round-1-disassembly/tools). Static paths that hit unofficial opcodes not observed are rejected (none remain).
- Data: banks 0-3 via descriptor extents (Image), shared pages bank2 $B200-$B2FF / bank3 $8000-$80FF also read by row copier $C7BB -> generic Data; banks 4/5 generic data; fixed/bank-6 tables each tied to referencing instructions (see map evidence strings).

### Classified / unknown
- Code 24731 bytes (Observed 13122, Static 11609 incl. 306 bytes of unreferenced fragments at 0.75 confidence), Image 65008, Data 41316, Text 32; Unknown 1 byte ($C415).

### Open questions (see leads.json)
- L-001 $C415 lone RTS byte; L-002 review unreferenced fragments; L-003 ending route; L-004 bank 4/5 semantics; L-005 $F161 caller context; L-006 test-mode options.

## Round 2 disassembly (checkpoint pass-002b)

### Classified / unknown
- Code 24731 bytes (Observed 17653, Static 7078), Image 65008, Data 41316, Text 32; Unknown 1 byte ($C415). Classification unchanged in
  extent; provenance and data roles refined.

### Findings
- Banks 4/5 sub-structure now typed regions with extent evidence (BlockAttr/BlockDef base tables at $F866-$F875; attribute counts
  220/127/124/90 equal definition counts; max map block id per page group = count-1; sound tables bounded by consumer index domains
  and pointer target minima). SndSequenceData end at last non-zero byte $BC67 is not an exact stream-end proof (both sides Data).
- Bank 6 frame data: $8A1D-$8A40 (unreferenced by frame pointers) decodes like a truncated stale copy of bank-6 object code
  (mid-instruction at both ends, never fetched); kept as data inside MsprFrameData.
- Weapon switching: Start in stage cycles sword -> weapon A ($0512=$02) -> weapon B ($01) when WeaponsOwned ($06FC) allows;
  AltWeaponA/B code observed (artificial ownership patch).
- Reachability: all round-2 Observed upgrades except test mode come from patched states; each region's evidence names the scenario.

### Open questions
- L-001 ($C415) blocked. L-002 commentary label wording. Boss rooms of stages 3-6, reward/clear path and remaining Static guardian AIs
  ($A8DF flyer, $A3BE keep-range, $AC48 BossStage3) unobserved (scenarios.json S-BOSS3-6) - structural classification already complete.

## Round 3 disassembly (checkpoint pass-003)

### Classified / unknown
- Code 24731 bytes (Observed 21962, Static 2769), Image 65008, Data 41316, Text 32; Unknown 1 byte ($C415). Extent unchanged.

### Findings
- L-008 applied: per-run evidence distinguishes artificial RAM-patched contexts (S-BOSS3/4, S-GUARD, S-MAPSEL, S6-PROG,
  S7-WALK2, S2-LIFT) from forced PC entry (S-BOSS5F/6F only: 108 bytes of BossStage5/6, BossAiStage5/6 and LoadStage 5/6 setup).
  Forced-entry execution shows mechanics only; natural reachability of stage 5/6 boss rooms is statically excluded (L-009).
- Test-mode SPTEST (A/B cycle, wraps at $25/$FF, sprite $18 companion object) and BGTEST paths executed on the natural cheat route.
  TestBgNum did not change on Up/Down in BGTEST during this run (input path observed, value effect not analysed further).

### Open questions
- L-001 ($C415) blocked. Remaining Static code: guardian fighter variants ($9CA3), dragon input branches ($B1ED/$B1AB),
  $ABF5, $F2DF, $B9C1, $C3C0, IRQ $C0D8, $D770, L-002 fragments - provenance-only questions.
