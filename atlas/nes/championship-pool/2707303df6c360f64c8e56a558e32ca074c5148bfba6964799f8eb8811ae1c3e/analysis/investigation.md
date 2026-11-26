# Investigation ledger

## Round 1 disassembly (checkpoint pass-001-final)

### Architecture facts established
- UxROM: bank shadow in zp $1E (also $5C saved bank). SetPrgBankFromA $E4B2 (`and #7; sta $1E; sta $FFF0,x`
  bus-conflict write), SelectBankN stubs $E4CE..$E538 (constant banks 2..6), bank-select table $FFF0-$FFF7 = 00..07.
- Trampoline family in the fixed bank ($F85D.., $FC35.., $FC51.., $FE9C..): save bank, select bank, `jsr $8xxx/$Bxxx`,
  restore. FarCallViaZp04 $E559: bank in $06, address $04/$05, via `jmp ($0004)` at $E569; callers load $04-$06
  from RAM $0435-$0437 (callback hooks) or from ($5F),y records -> dynamic targets (only observed ones known).
- Vectors: NMI $D73D, RESET $E3E7, IRQ $D880. Reset uses txs at $E3F6/$E412 only.

### Methods and outcomes
- Static: analysis/tools/walk.py (conservative bank-aware traversal; JSR followed; stubs modelled; banked code
  calling a stub stops; unofficial opcode = path rejected). From vectors alone: 40.6 KiB; with observed roots added:
  42.8 KiB. All 10,457 observed instruction starts lie on walked instructions; 0 bank-context mismatches.
  No overlapping decodes, no BRK in walked code.
- Observed: trace aggregates (analysis.contexts) dumped to analysis/observed-roots-r1a.json (boot, menus,
  tournament setup, auto break, in-game menus) and -r1b.json (+ Challenge and Party setup).
- Gap fill: analysis/tools/gapfill.py accepted gaps >= 8 bytes between known code exactly filled by a clean decode
  (998 bytes, Static, confidence 0.7, reachability unknown). Smaller fills left unknown (L-006).
- Data: banks 0/1/4 and bank 6 $8000-$B152 classified generic data from (a) no control flow entering them,
  (b) per-1 KiB and per-256-byte linear-decode statistics (unofficial 10-71%, BRK up to 64%; known code ~0%),
  (c) tile-structured renderings (analysis/tools/tiles.py) for banks 0/1, (d) observed data reads in scenario S6
  (analysis/observed-reads-r1-b01/-b4/-b6.json). Bank 2 $8000-$B1D2 is ASCII text/script data.
  Tables in code banks classified only where referenced by indexed loads or clearly patterned (palettes, curves,
  1 KiB smooth table, nametable runs). Exact table layouts are not claimed.
- Padding: uniform $FF bank tails (banks 0,2,4,5,6,7).

### Remaining unknown (2,225 bytes) and why
- Bank 3 (2,045): $830E-$8325, $832B-$866B, $88A8-$89D6, $8A51-$8B13, $95F6-$9880 contain code-like content without
  known callers mixed with tables; code/data boundaries not yet established (L-001..L-004). Plus 1-3 byte fragments.
- Bank 7 (145): unreferenced trampolines $F885/$FB32/$FC65 (L-001), $E3A3-$E3E6 (L-005), 3 single bytes.
- Bank 5 (32), bank 2 (3): small fragments (L-006).

### Rejections / non-decisions
- None rejected. Lone RTS/JMP fragments deliberately not accepted on exact-fill alone (insufficient structure).
- Shot input not established (S5 attempted); not a classification rejection.

## Round 1 redirect (attempt analysis/redirect/r1-a7)
- Bank-6 screen scripts decoded (analysis/redirect/r1-a7/scripts.py, bank6-screen-scripts.txt): far-call/hook entries
  for every bank-3/fixed unknown block: $832B, $830E/$831A, $F885/$F899, $95F6. TitleSkipWord writer = $832B routine.
- Runtime: boot trace (natural) observed $830E-$8325, $832B-$8644, $FC65; opponent-message trace observed $F885-$F8AC,
  $899E-$89C0, $8A51-$8A74, $E3A3-$E3E1 (route had one RAM patch); intermission (artificial GameCmd $19) observed
  $95F6-$981F starts. All observed starts on static walks.
- Table extents with consumers for $8645, $88A8-$899D, $8A76, $8B00, $977D-$97B2, $9820-$9880, $E3E2.
- L-007 resolved: call-shot rule explains failed shots; tournament RAM contract ($EF/$E6/$DD) reached round 5 and ending.
- Outcome: L-001..L-005 ready with exact ranges (~2,139 bytes), L-006 families (~86 bytes), L-008 commentary-only.
  Details: analysis/redirect/r1-a7/notes.md, analysis/handoffs/round-1-redirect.md.

## Round 2 disassembly (checkpoint pass-002)
Input: pass-001-final (2,225 unknown). Tool: analysis/tools/annotate_r2.py (dry run by default; --apply), provenance
upgrade analysis/tools/upgrade_r2.py, traced ending driver analysis/tools/r2_ending.py.

### Lead consumption (all re-checked before annotating)
- Code ranges were annotated only from bank-aware static walks (analysis/tools/walk.py) rooted at the script-decoded
  entries; each range had to tile exactly with official opcodes and no overlapping decode. Observed provenance only
  where the redirect trace (analysis/redirect/r1-a7/observed-*.json) recorded a fetch at that instruction start with
  the same effective bank; the remainder Static (conf 0.9 on walked paths, 0.6 for uncalled contract routines).
  Artificial-route status is written into each region's evidence (S8b CalledPocket patch; S13 GameCmd $19 patch).
- Data tables: in-bank absolute reads re-counted from the static instruction set for every proposed table
  (e.g. $8645: 8 reads lda $8645..$864C,x; $8665: 7; $8B00: 11; $88E8: 2; $E3E2: cmp $E422/lda $E485).
  $88FE-$8992 (names) and $8A76-$8AFF have no direct absolute reads: names reached through the $88E8 pointers;
  $8A76 classified only as generic data (no reference, non-code decode, uniform runs).
- CORRECTION to redirect L-003 bounds: the $97A3 word table has 7 entries ($97A3-$97B0). Index = 2*($977D value),
  values 0-6; the would-be 8th word is $4B20 (not a bank-3 pointer), and the loop "jsr $E54B; lda $2E; and #$08;
  beq; rts" begins at $97B1 because its beq targets $97B1 exactly. Annotated data $97A3-$97B0, Static code $97B1-$97BA.
- L-006 decided per family (all Static code, conf 0.5-0.7, reachability none claimed):
  * dead else-skip JMPs in bank-3 rules sections ($A394,$A3A4,$A88E,$A89E,$A9E1,$AABD,$AB42,$AC88): each sits between
    a JMP and the next branch target and jumps just past it - the same compiler if/else skeleton 8 times;
  * dead JMP after RTS/JMP: b2 $B502, b3 $A863, $AD85 (targets are known code);
  * dead epilogue RTS (b3 $9D83,$A8A4,$A8EA,$AAC3,$AB48,$AF2F,$B1A3,$B7ED,$BA2C,$8A75; b5 $BCC8; fixed $DC06):
    independent support - 6 reachable "rts; rts" pairs exist in classified code whose second RTS is an early-return
    branch target, i.e. the compiler always emits a final epilogue RTS; these are the instances without such a branch;
  * bank-5 "lda #n; ora #n; rts" stubs after JMP ($B285,$B29B,$B2C2,$B2D8; n=1,4,3,6) and tails $9185, $AA9A;
  * b5 $8A48: NOT dead - ordinary RTS after jsr RestoreSavedBank $E526 (walker stops at bank-restore calls);
  * prologues: $DE1C pha (VramBufFlushNow ends by pulling the A its callers pushed; direct entry needs this pha),
    $D9B0 txa into UploadPalette's pha;
  * $8469/$8476 "lda #$80; sta $6C" dead else-arms inside the observed title routine.
  Competing interpretation considered: stray data bytes. Rejected for these because each lies inside contiguous
  code with no in-bank data reference, decodes exactly to the gap, and matches a repeated compiler layout; the
  claim is structural only (no reachability).
- Provenance upgrade: 28 Static code regions (748 bytes) whose every instruction start appears in the redirect
  traces became Observed (offset/length unchanged; previous evidence text kept). 25 partially observed regions
  left Static (no splitting, to keep commentary anchors stable).

- Round-2 gameplay: traced ending route S14 (analysis/observed-r2-ending.json; artificial RAM shortcut entry):
  4,976 starts, 0 off classified instruction starts; upgraded 10 more fully observed Static regions (223 bytes).
- Workspace tool fix: analysis/tools/rpcd.py daemon crashed (BrokenPipe) when a client was killed mid-call by a
  timeout, losing unsaved live state; now ignores client write errors. Re-applied annotations (idempotent scripts).

### Result
ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown). Checkpoints pass-002 (leads only)
and pass-002-final (+ S14 upgrades) both ok, byte_identical (analysis/pass-002-final-verification.json).
Code Observed 23,556 / Static 22,188; Data 84,476; Padding 868. Low-confidence code (conf < 0.75): 878 bytes in total
(round-1 exact gap fills + round-2 L-006 fragments + uncalled $FB32 and $97B1 routines); these are the first
candidates to revisit if a contradicting data consumer is found.

## Round 2 redirect (attempt analysis/redirect/r2-q1)
Input pass-002-final (100.00% classified). Classification-quality / rejection review before final acceptance.
- Static xref of all 56 low-confidence code regions (878 B): no data reader inside any of them; all nearby indexed
  tables have bounded domains (e.g. $97A3 words max offset 13 -> ends $97B0; PaletteTable 7 entries, script args
  0..6). No control flow or observed fetch into any Data/Padding region; only one code-bank data block decodes
  cleanly ($9790, consumer-anchored). $8A76-$8AFF: well-formed 16x8 tile-rect + 8 attribute bytes, unreferenced:
  keep generic data. $DE1C pha supported by VramBufFlushNow's 2-push/3-pull stack contract.
- Outcome: keep all (0 revert, 0 boundary fix). Stale metadata found (989 callers/observed, 974/1112 reachable
  fall-through, S14 wording) -> L-009. Commentary findings (NameCharMap extent/role, credits op 07, PortraitFrame
  1/2 meaning, misnamed DelayThenEnding path, RandomMessageTable entry 0) -> L-010.
- Runtime: credits entry state (natural), char-map read trace (bank-7 font data), WorldChampUnlock consumer
  (artificial), $DD=2 shortcut skips EndOfMatch. Notes: analysis/redirect/r2-q1/notes.md.

## Round 3 disassembly (pass-003, metadata only)
Input pass-002-final (100.00% classified, 0 unknown). Consumed ready lead L-009 with
analysis/tools/annotate_r3.py, which re-checks each fact from the ROM and the saved traces before editing:
- Region 989 bank 7 $F9A9-$FA3E (DrawBigNumber): the five `jsr $F9A9` in bank 3 ($96E8, $970B, $972E, $974C,
  $9772) are the only jsr/jmp references in the ROM. 74 of 77 instruction starts are in the ARTIFICIAL S13 trace
  (unobserved: $F9C4, $F9C7, $F9C9). Evidence rewritten and confidence 0.7 -> 0.9. Provenance stays Static
  (partial observation; artificial-state wording kept).
- Regions 974 and 1112, bank 5 $8A3D-$8A48: the fall-through of AimAtCalledBall ($8A3A, region 436) after a
  returning jsr SelectBank5_Temp. They are statically reached; 0.7 -> 0.9.
- Region 1122 $DE1C: evidence now states the VramBufFlushNow stack contract (2 pushes, 3 pulls); 0.5 -> 0.65.
  Reachability is still not claimed.
- Wording: the 10 S14-sourced Observed regions now say "unpatched execution ... after that artificial shortcut;
  not natural reachability". The 6 opponent-message regions now label "natural reachability appears likely" as a
  judgement, not evidence.
- Structure: offsets, lengths, kinds, provenance and views are unchanged on all 20 edited regions (checked by
  diffing map.get before and after). No other easy gain is available because 0 bytes are unknown.
- Result: ROM classification: 100.00% (131088 / 131088 bytes classified; 0 bytes unknown). pass-003 is ok and
  byte_identical (analysis/pass-003-verification.json).

## Round 3 redirect (attempt analysis/redirect/r3-p1)
Input pass-003 (100.00% classified, 0 unknown). Commentary questions plus the Challenge milestone. Map/state/source
untouched. Details: analysis/redirect/r3-p1/notes.md; handoff analysis/handoffs/round-3-redirect.md.
- PortraitFrame 1/2 visually confirmed: 1 = scowl (opponent lost), 2 = grin (opponent won). The entry was artificial:
  GameCmd $D2=$12, which is GameCmdDispatch's `jsr RecordGameWin`, plus PlayerWins/CurrentPlayer writes. After the
  write, the game's own EndOfMatch screens ran.
- RecordGameWin entry 0 confirmed: RandomLo forced to &$1F=0 at PC $B1F9 gives Ptr04=$9C2F ("SO, YOU THINK...").
- FontTablesTail is not unread.
  - $CDC3-$D0C3 is WindowTileSets record 3 (bank 7), a count-prefixed 48-tile CHR set (769 bytes) that ends at
    PlayMatch. OpenWindow uploads it for window frame record 1, the SET POWER panel. All 769 bytes were read in a
    natural trace.
  - $CDAE-$CDC2 continues NameCharMap (codes $6B-$7F) and was never read.
  - No code evidence: the data classification is confirmed.
- Bank-3 $8A76-$8AFF (region 1064) is the SET POWER window rectangle. PowerMeterPanel sets WinParams=$8A76/$8AF8, and
  all 138 bytes were read. This corrects the r2-q1 "no reference" premise, which rested on an operand-only check.
  SpinPanel's rectangle starts Data region 998 ($9043).
- Challenge Speed Pool (artificial ball clear) ran the game's own WorldChampUnlock increment and the EVALUATION
  screen.
- Leads: L-011 (map evidence, ready, low) and L-012 (commentary, ready, low). There is no classification change.
  Recommendation: final acceptance.

## Round 4 disassembly (checkpoint pass-004)
This pass applied lead L-011 as metadata only. The scripts are analysis/tools/annotate_r4.py and
analysis/tools/splitimm_r4.py. annotate_r4.py re-asserts every fact from the ROM and the saved r3-p1 traces before it
edits anything. Kinds and bytes are unchanged.
- **Region 1016 split at phys $1CDD3.**
  - The constraints are independent: the count byte (48), the count-prefixed 16-byte tile contract
    (WindowTileSets record 3 = $CDC3, bank 7, 4-byte records at bank-3 $801C), the PlayMatch code at $D0C4, and a
    natural read of exactly $CDC3-$D0C3.
  - 1016 is now $C86D-$CDC2 (1366 B, Data, Static 0.9). Its readers are AsciiTileTable, BigFontTop/Bottom and
    NameCharMap $CD43, which is set both by the WindowFrameTable record-0 char-map word and by split #imm stores in
    bank 2/3. The tail $CDAE-$CDC2 is NameCharMap codes $6B-$7F; this is a pattern-only reading and was never
    observed read.
  - The new region 1131 is $CDC3-$D0C3 (769 B, Data, Static 0.95): the SET POWER window CHR tile set.
- **Region 1064** ($8A76-$8AFF) is the SET POWER window rectangle. PowerMeterPanel sets WinParams with split #imm
  stores at $8B21/$8B30 for DrawWindowRect, and all 138 bytes were read naturally. The "no reference found" wording
  was removed and confidence went from 0.9 to 0.95.
- **Region 998**: the evidence now records that it starts with the SET SPIN rectangle (SpinPanel $8E44/$8E4E,
  header (10, 8), attributes at $9095). This is static only.
- **Split-immediate scan** (analysis/round-4/splitimm.json). It covered 33 regions: every region whose evidence says
  "no reference / no caller / unreferenced", plus 998, 1016 and 1064.
  - Patterns searched: stored pairs (`ld? #lo; st? M` ... `ld? #hi; st? M+1`) and loose #imm pairs. For Code
    regions the targets were the start and start-1. Hits were filtered for bank relevance.
  - Code regions: none of the 28 low-confidence Code regions got a hit. This agrees with the r2-q1 immpair check
    (xref-lowconf.json), and their reachability wording stands.
  - Region 992 (padding): no hit.
  - Region 991 (bank 0): fixed-bank code stores 44 $8000-$BFFF address pairs. Their effective bank is not statically
    resolved, so none is attributed to bank 0. The wording "no code reference enters it" was narrowed to "no code
    flow or direct operand", and the evidence now points to the S6 runtime reads.
- Verification: map.validate is ok. state.json was saved, maps/current.json exported, and pass-004 is ok and
  byte_identical (analysis/pass-004-verification.json). Classification: 100.00% (131088 / 131088; 0 unknown).
- Toolkit: TK-004 is a doc mismatch (on NES, map.annotate needs the `region` form; id 0 creates a region).

## Round 4 redirect (attempt analysis/redirect/r4-m1)
Input pass-004 (100.00% classified, 0 unknown). This pass covered commentary's three optional questions with natural
input only (no RAM writes). Map/state/source untouched. Details: analysis/redirect/r4-m1/notes.md; handoff
analysis/handoffs/round-4-redirect.md.
- **BallMarkerSprite (bank 4 $B209):** confirmed in bank-4 context.
  - Static: the only path is BuildSprites -> SelectBank4_Temp -> DrawBallSprite (Tmp09=1) -> DrawBallMarker, with no
    bank write in between.
  - Runtime: all 2079 reads of phys $13219-$14000 during ball calling (GameCmd $10) had prg_banks [8,9,14,15].
  - $B209 is a set-pointer word pointing to the record at $B5D8. The block holds several metasprite records.
- **SET SPIN window (region 998):** the natural trace read exactly $9043-$909A (82-byte rectangle + 6 attribute
  bytes). The $909B-$90BA run was unread.
- **Speed Pool total is elapsed seconds** (ShotClockTick -> GameFrame -> PlayerScore, TIME heading, unlock < 141).
  ChallengeResult does not invert it, which is a likely game quirk.
- **Leads:** L-013 (commentary, ready, low). L-008 was closed as already applied in round 2.
- No classification, bank-context or boundary change. Recommendation: final acceptance.
