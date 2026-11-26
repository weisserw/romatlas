# Redirect attempt r02-a1 (round 2) — research record

Input checkpoint: pass-002 (maps/pass-002.json == maps/current.json; analysis/state.json), 100.00% classified
(262160/262160, 0 Unknown). No map/state/source change in this attempt. Scripts in this directory run from the
workspace root. Runtime bridge socket: /tmp/ra_r02a1.sock (tools/bridge.py; stopped at the end).

## 1. Whole-map static consistency audit (audit.py -> audit.txt)
Bank-aware linear decode of every Code region (18316 instructions) against the pass-002 map:
- A. JSR/JMP absolute targets not on a decoded Code instruction start: 0.
- B. Branch targets not on an instruction start: 0.
- D. Undefined opcodes inside Code: 0. G. Decode overrunning a Code region end: 0.
- C. Code runs ending in a conditional branch that could fall into non-Code: 6, all checked and benign
  (LDA #imm before BEQ/BNE makes the branch always taken: bank1 $99C9, bank3 $91B5, bank5 $94D9, bank5 $ABA3,
  bank6 $AFEA; fixed $C55A `INC MtileStrip / BNE` with MtileStrip 0..7 so it never wraps).
- E. Observed fetches (scratch/cov.json + r01-a1/cov_session2.json, 14188 starts) outside Code or off a decoded
  start: 0.
- F. JSR/JMP to a known routine entry inside non-Code: DR_StaleForeignImage (expected, L-001), five `20 00 FF`
  byte triples inside bank 3 $B4A1-$BEA6 noise data (uninitialised-EPROM-like $FF/$7F/$BF bytes; coincidence) and
  **bank 3 $8610** (see L-013).
Conclusion: no code/data contradiction in the established Code graph. Remaining findings are local.

## 2. Fall-in / island scans (fallin.py -> fallin.txt, islands2.py -> islands2.txt)
fallin.py: non-Code bytes immediately before a Code region that decode cleanly into it. islands2.py: non-Code
regions that decode cleanly from start to an RTS/RTI/JMP at their end.
Inspected hits:
- **L-013 bank 3 $860E-$8614** (file 0xC61E-0xC624): `LDX #$01 / JSR DR_DrawRacers($8615) / LDX #$00`, then
  falls into DR_DrawRacers. Currently the last 7 bytes of the Data run labelled DR_ActionRiderX.
  Independent extent proof: DR_ActionBase = 0,1,2,5,6,9,12,28,30,46,50,51 and DR_ActionTicks = 1,1,3,1,3,3,
  16,2,16,4,1,32; base[i]+ticks[i] = base[i+1] for every i and 51+32 = 83, so the parallel action tables
  (index = DR_ActionBase[DrAnimFrame] + DrAnimTimer, DR_DrawRacers @riderFrame) have exactly 83 entries.
  DR_ActionFrame ($8D48, 83 B, ends at DR_ActionBase) and DR_ActionRiderY ($8568-$85BA, 83 B, ends at the
  DR_ActionRiderX operand) already agree. Hence DR_ActionRiderX = $85BB-$860D and $860E-$8614 is not table.
  Same "both players" idiom as observed LatchBothPads ($C117), DR_SetVelocityBoth (L-009, accepted) and
  FS_FlushPendingSfx (bank 2 $834E, observed). No reference to $860E (refscan; no word/JSR). Reachability:
  unreferenced (DR_Main's caller path calls DR_DrawRacers with X = FrameCounter&1).
- **L-017 bank 0 dead fragments** $811A-$8121, $817A-$8181, $81C3-$81CA (file 0x12A, 0x18A, 0x1D3; 8 B each,
  Data 0.7): each is `LDA #$01 / STA FrameTimer($0625) / STA $0626` sitting after `JSR CheckSkipButton /
  BEQ +$0B / JMP NumPlayersMenu($81D8)` and falling into the observed `LDA FrameTimer / BNE` wait loop.
  The BEQ skips them and nothing references them (audit B/A). Same structure as L-011 multi-instruction
  dead fragments that disassembly accepted as Static Code conf 0.7. Optional fidelity only.
- **L-018 bank 2 $8363-$8372** (file 0x8373-0x8382, 16 B, Data 0.7): `LDA #$00 / CLC / ADC #$20 / STA $8364 /
  AND #$C0 / ORA #$18 / STA PPUMASK / RTS`. The STA operand $8364 is exactly its own LDA immediate byte
  (self-modifying emphasis cycler), which anchors the assembly origin at $8363 in this bank. It sits between
  FS_FlushPendingSfxX (ends JMP $AB52) and FS_PlayerLogic ($8373). Unreferenced. Executed from ROM the store
  would be an MMC1 serial write. Competing reading: leftover debug code that cannot work in ROM (keep Data).
  Optional, low; disassembly decides between Static Code (conf 0.7, documented as non-functional in ROM) and
  keeping Data with a commentary remark.
- Others were data: bank 4 $8C34 (32 B of table before an LDX/JSR idiom already Code), bank 1 $8EAD ($EE run),
  bank 2 $957C (table), bank 1 $BDF1, bank 4 $8376 (`BIT $2002` before observed loop $8379; one 3-byte
  instruction with no reference — too little structure, left Data), islands2 hits at bank 2 $9FBE, bank 3 $B652,
  fixed $D7D3 and $FD44 (music/tile streams).

## 3. Text/table boundary error at the Caves of Fame titles (L-014)
Consumer (bank 0, Caves of Fame page loop): `LDY FameEventIdx / LDX FameTitleOffsets,Y / LDY #$14 /
@title: LDA FameTitles,X / STA PPUDATA / INX / DEY / BNE`. FameEventIdx takes 0,1,2,3,5 (4 skipped, 6 restarts).
FameTitleOffsets ($8547) = 00 14 28 3C 3C 50: six entries; entry 5 ($50) is used. FameTitles = $854D + up to
$50 + 20 = $854D-$85B0 (100 B, five 20-byte titles).
Current map: region 35 (Text, 0x55C+8) starts with the $50 offset byte ($854C, 'P'), and region 41 (Text,
0x5AF+19) ends with 0x5C1 ($85B1, $4C) which is the opcode of `JMP $85B1` (operand $B1 $85 is region 42,
Data). Self-jump `4C B1 85` at $85B1 is the same hang idiom as bank 0 $89ED, bank 2 $902F and bank 3 $8987
(all Data by precedent). The 20-byte title blocks are also split into Text/Data fragments (regions 35-41).
Proposal: Data FameTitleOffsets 0x557-0x55C (6 B); Text (or Data) FameTitles 0x55D-0x5C0 (100 B, 5x20,
$03 = blank tile); 0x5C1-0x5C3 one item (Data by the self-jump precedent, or Code if disassembly prefers).

## 4. Commentary factual error (L-015)
bank7.asm IrqHandler header and uncertainties.md say "only bank 1 executes CLI, and bank 1's own IRQ vector is
a bare RTI". Source shows CLI in bank 1 (MT_ScreenOn), bank 4 (SR init) and bank 5 (CL init). The per-bank IRQ
entry `JMP` at $8003 goes to $8074 (bank 1), $804E (bank 4, RTI, observed fetch file 0x1005E) and $803D
(bank 5, RTI, observed 0x1404D); banks 0, 2, 3 and 6 jump to IrqHandler $C000. IRQs therefore occur (observed
RTIs in banks 4/5, DMC IRQ armed by NMI), but only in banks whose handler is a bare RTI; banks routing to
$C000 never execute CLI (WarmStart/EnterNextEvent execute SEI). The conclusion (IrqHandler static-only) stands;
the stated reason must be corrected. Commentary-only fix.

## 5. Multi-player mode (S15, L-004 reachability)
Route (natural inputs only, no RAM writes): analysis/snapshots/mainmenu.json -> Restart ([down] 2f+25, [a]
2f+300) -> Number of players: hold Up until SprTextDigits ($04E8) = '0'+N, [a] -> each player p picks a free
character on keypad 1 (even p) / keypad 2 (odd p; the screen shows "PLAYER n KEYPAD k"); each pick needs two
A presses (select + biography) -> main menu -> Practice ([a]) -> event select.
Scripts: mp_np.py, mp_chars*.py, mp_full.py, mp_event.py, covcheck.py.
- 3 players: main menu snapshot mp3_mainmenu.json. Practice Clubbing ([down]x2, [a], 900 frames of A/Right on
  both pads): **CL_Bracket3 (L-004 root 0x154A1) fetched**. Session coverage cov_mp3.json: 5362 starts,
  118 new vs prior unions, 0 outside Code/off-start. (The earlier extra A presses also started a 3-player
  Saber Race, "GLUNK VS CRUDLA", no root hit.)
- 6 players (fresh server, trace off for menus): picks needed D-pad moves when the cursor rested on a taken
  character (Right from column 2 does not wrap; mp_pick.py steers toward the first untaken one). Main menu
  snapshot mp6_mainmenu.json. Practice Clubbing (600 frames): **CL_Bracket6 and CL_PairFromWinners fetched**.
  cov_mp6.json: 2802 starts, 0 outside Code.
- Union of both sessions vs prior unions: 144 new instruction starts (cov_new_r02.json) in 21 Static Code
  regions (bank 0 multi-player character select/menu, bank 4 3-player Saber Race, bank 5 brackets). Proposed
  provenance upgrade: L-016. CL_Bracket4/5 not run: same NumPlayers selector (CL_InitBracket indexes
  CL_BracketHandlers by NumPlayers); repeating N=4/5 would only replay the demonstrated mechanism (recipe:
  mp_full.py/mp_pick.py with N=4/5, then mp_event.py).
- Remaining L-004 roots (AttractAltScreen, DrawMenuLowerScreen, CL_DrawStaminaBoth, UpdateBestScoresHigh,
  SfxOnlyUpdate, MT_RankByDistance, DR_AdvanceCourse): no absolute, word or (word-1) reference in their bank or
  the fixed bank (scan in this attempt; only LDA #hi coincidences). Not hit by multi-player runs. No selector known.
- Performance note: with trace enabled the session slowed sharply over time (TK-002, already recorded);
  menus were driven with trace off. A stale serve was stopped and restarted once; no other effect.

## 6. Avenues considered and deferred
- Natural full Start-the-Games run to final placings (S16): deferred; placings/results/Caves of Fame already
  visited, all events finished naturally; would only re-run observed paths (see scenarios.json S16).
- Bank 2 $8A9D 612-B sprite-like block, bank 3 $AEF9-$B01D, IrqSplitParam odd bytes: classified Data; meaning
  only (L-005). No consumer found in earlier inspections; no code-vs-data question.
- Raster IRQ path $C000: static-only; reachability requires I clear while bank 0/2/3/6 is mapped. All CLI sites
  are in banks 1/4/5 whose handlers are bare RTIs; WarmStart/EnterNextEvent SEI. No natural route; not pursued
  further (L-015 corrects only the stated reason).
- Bank 4 $8376 `BIT $2002` (3 B Data before the observed $8379 loop): one instruction, unreferenced, lacks
  structure; left Data.
