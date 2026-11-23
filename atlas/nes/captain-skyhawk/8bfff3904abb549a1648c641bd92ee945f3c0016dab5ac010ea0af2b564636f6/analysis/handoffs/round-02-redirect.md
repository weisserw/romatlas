# Round 2 redirect handoff (attempt r02-a) - COMPLETE

- Input checkpoint: pass-002 (maps/current.json = analysis/state.json sha256 6fd86fe7...c0df), round-2
  commentary complete. ROM, map, state, source and binary unchanged by this phase (hashes re-checked).
- Research dir: analysis/redirect/r02-a/ (tools/, frames/, snapshots/, audit.txt, datarefs.txt,
  lonerts.txt, e2-e5/e3c logs, leads-before.json, scenarios-before.json). r01-a reused by reference only.
- Classification: 131088/131088 classified, 0 unknown (unknown complement empty; no ledger refresh needed).

## Scope
1. Whole-map consistency audit for possibly WRONG classifications (code<->data).
2. "No reader" data (1:$D649=3:$D7B8, 3:$D76E, 3:$D90A, 2:$FE65, 3:$F1C3, 1:$FE16).
3. Carrier landing autopilot ($2C/$44) observation; controller-2 B+Down; L002 toolkit status.

## Findings
- Audit (tools/audit.py, audit.txt): all 487 regions; every Code region decodes with official opcodes and
  ends exactly on an instruction boundary; all 2930 jsr/jmp/branch targets land on Code instruction
  starts in the same 32K bank (0 in data, 0 misaligned); RAM targets only $0700/$079C (known L002 images);
  only fall-through into Data is the bank-1/2 NMI stub after `sta $FExx` (AxROM bank switch to bank 0 -
  correct). No Data region contains jsr/jmp to established entries beyond chance (tools/datascan.py).
  Only other lone $60 after a terminal instruction at a Data start: 3:$D76E (lonerts.txt).
- L009 (new): bank-1 BlockSubstTables is 128 bytes, not 48. LoadBlockSubst reads $D600+16*Mission for
  Mission 0-7 (static contract; 8-entry per-mission tables; all MissionGfxSet handlers jmp LoadBlockSubst)
  and E3c observed exactly those 16-byte reads for every mission index 0..7 (artificial Mission patch).
  So "no reader" 1:$D649 and fills $D630/$D670 are mission 3-7 records ($D640-$D67F == $D600-$D63F).
  Bank 3 $D76F-$D7EE is a byte-identical unread copy (no operand addresses it); 3:$D76E is a lone $60
  after GroundUnderAt's RTS.
  Caution for future work: Read traces include 6502 dummy reads (RLE decoder page-cross reads of
  $D600-$D67F; RTS dummy read of $D76E) - not consumer evidence.
- S11 (visited, artificial): mission 4 (index 3), SectionIndex:=5 + TerrainPos:=$03B0 -> section $44,
  MissionStage:=1 -> autopilot descends, section list wraps $44 -> $2C (carrier deck), plane lands
  (OnDeck=1, speed 0), marker object $5D/$5E reaches GroundObjRow $68 -> GroundObjs_Landed,
  MissionEndMode=1 -> climb-out -> $DE6A briefing. Coverage confirms 0:$92A0-$937E and 3:$D8A8-$D909.
  Commentary can drop "read statically, not observed" for Flight_AutoLand / UpdateGroundObjects landing,
  and should state: missions 4 and 8 are the carrier missions ($2C start/landing deck, $44 approach).
- S12: controller-2 cannot select the phase at mission start (Briefing_NextPhase overwrites Joy2Held with
  the saved code); no ground handler tests $44 (B+Down) - only B+Up $48 and StarSpeed bits in space.
  Closes commentary question 2.
- L002 still blocked: toolkit docs/nes.md:91 "Uploaded execution views are currently rejected".

## Leads (analysis/leads.json, pass record added)
- L009 READY / medium: region merge/split, 0 byte-count change, corrects table extent and removes three
  "no consumer" claims. Recipe in leads.json (static; evidence already captured; no replay needed).
- Updated: L002 (blocked, re-checked), L005 (bounded re-inspection, remaining items semantics-only).

## Recommendation
One short disassembly pass applying L009 (region edit + checkpoint + verify), then commentary update for
L009, S11 and S12 (uncertainties items 1, 2, 6; Flight_AutoLand / GroundObjs_Landed / DataB1_D6xx /
DataB3_D7xx notes). After that no further redirect avenue is open: hand off to final acceptance
(editorial audit) using the rejection review below unless new evidence appears.

## Rejection review (deferred/rejected avenues)
| Avenue / claim | Decisive artifact | What it rules out | Strongest alternative | Disposition / reopen condition |
|---|---|---|---|---|
| Misclassified code/data anywhere | r02-a/audit.txt, datascan.py | targets into data, misaligned code, unofficial opcodes, unexplained fall-through | data that happens to decode cleanly but has no reference | closed; reopen on a concrete counterexample (reference into data) |
| 12 no-caller code ranges (L006-L008), weak lone RTS 0:$844C/0:$E0B7/3:$91B9 | r01-a/refsearch.txt, evidence/round-02/frag-verify.json | none falsified; no reference found by word/word-1/imm-pair search | 1-byte data/padding for lone RTS | keep (conf 0.6); no runtime test can discriminate; reopen only if a data consumer reads them |
| 3:$D90A (8 B), 2:$FE65 (338 B metasprite-shaped), 3:$F1C3 (202 B spare height maps), 1:$FE16 | r02-a/datarefs.txt, tools/dupscan.py | same-bank absolute/indexed readers, imm-pair pointers, byte-identical copies elsewhere | unread leftovers (current notes) | semantics only, Data correct; reopen if a pointer table/descriptor is found targeting them |
| RLE overlaps (1: stream $BA at $80C9; 0: $CA01..$CB07 over $CAF8) | commentary/uncertainties.md item 6 (decoded lengths) | n/a - both readings are data | shared bytes vs miscount | no classification effect; meaning-only |
| L002 RAM images 0:$F796->$0700, 2:$C730->$079B | toolkit docs/nes.md:90-92; r01-a coverage (RAM PCs only in the two images) | other RAM-executed ROM sources | Code-with-upload-view | blocked on toolkit feature (missing, not a ROM question) |
| Controller-2 B+Down on ground | S12, e5.out; MissionStart/Briefing_NextPhase source | ground/start-phase effect of $44 | none | closed |
| Landing autopilot $2C/$44 | S11, e4_cov.json | (now observed) | - | closed |
| Natural (unpatched) mission completion | S01-S11 | - | - | patches already reach all milestones incl. ending; manual completion adds no structural evidence |
