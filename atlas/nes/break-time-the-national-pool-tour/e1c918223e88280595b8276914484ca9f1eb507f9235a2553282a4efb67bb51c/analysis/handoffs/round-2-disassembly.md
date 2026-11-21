# Round 2 disassembly handoff

## Checkpoint
- Input: pass-001 (state restored via map.load; baseline not re-created).
- Output: **pass-002** — maps/pass-002.json, generated/pass-002, analysis/pass-002-verification.json:
  ok=true, byte_identical=true, map sha256 671ba03cfdce93bc009541110a705db3def36fbcbcee488907d39e628de2863d.
  map.validate ok; analysis/state.json saved; maps/current.json exported (identical to pass-002 map).

ROM classification: 99.99% (163833 / 163856 bytes classified; 23 bytes unknown).

Status: **incomplete** (23 bytes unknown). Pass ended at easy-gain exhaustion (not interrupted).

## Per-bank counts (code / data+image / text / padding / unknown)
header 0/16/0/0/0; prg00 6202/1692/294/0/4; prg01 0/6476/0/1716/0; prg02 9/8183/0/0/0;
prg03 53/6788/0/1351/0; prg04 9/8183/0/0/0; prg05 53/7368/0/771/0; prg06 6931/1110/148/0/3;
prg07 3315/4106/0/771/0; prg08 1937/1625/241/4389/0; prg09 0/8/0/8184/0; prg10 5587/1628/966/0/11;
prg11 6103/197/514/1378/0; prg12 1004/7188/0/0/0; prg13 0/6476/0/1716/0; prg14 4090/111/38/3948/5;
prg15 0/16/0/8176/0; chr 0/32768/0/0/0.
Code 35,293 (Observed 19,530; Static 15,763). Regions: 720 Code, 435 Data, 140 Text, 8 Padding, 1 Image.

## Accepted / rejected per lead (bytes newly classified)
- L-010 DPCM: 26,330 accepted / 0 rejected.
- L-005 sound: 12,945 accepted (incl. 42 code) / 0.
- L-015 bank 0/1 screens: 4,817 / 0.
- L-014 bank 7: 2,822 / 0 (optional $BCC0-$BCF4 accepted as residue data, confidence 0.7).
- L-001/L-007/L-011 bank-0 closure: 908 code + 43 data (CharToTileTable) / 0.
- L-011 slots, reset stubs, bank 8/14 closures: 320 + 200 + 165 / 0. L-013 within these.
- L-012 bank 6: 241 / 0.
- L-016/L-008/L-009 residual ledger: 7,450 accepted (tables 4,765; strings 1,295; non-code gaps 590;
  adjoining 62; code 738 from 16 clean unreferenced/fallthrough routines) / 23 left unknown.
- L-002 superseded; L-003 verified naturally (SC-015); L-006 verified (SC-014).
Review evidence for each: analysis/investigation.md "Round 2".

## Remaining unknown (23 bytes, lead L-017)
Nine code-shaped fragments right after unconditional JMP/RTS with no reference: phys 4964 (3),
6173 (1), 51444 (3), 84584 (3), 84607 (3), 85771 (5), 117501 (1), 117621 (3), 118162 (1).
Likely unreachable macro epilogues/stubs; left unknown because a lone RTS/JMP lacks sufficient
structure (evidence-policy.md). None executed in traces.

## Gameplay
- SC-014 Trick Shot (traced). SC-015: whole tour (5 cities) and EndingSequence reached with natural
  inputs via the in-game A+B+SELECT result shortcut; stage-won transition and ending start traced.
- 5,044 contextual roots this round, 0 in Data/Text/unknown; 23 Static regions (544 bytes) upgraded
  to Observed. Evidence: traces/round-2/ (screenshots, cov.json, tools).

## Deferred
- L-017 (23 bytes): needs an independent constraint (dispatch-table reference or macro-pattern
  argument); redirect question, low value.
- WATCH / 2-player modes (SC-010): shared engine, no unknown bytes depend on it.
- Staff-credits display consumer (bank 10 text) not observed in the ending window traced.
- Specific layouts of the 42 consumer-anchored tables (classified as generic data only).

## Leads
Consumed/updated: L-001..L-016. Produced: L-017.

## Toolkit issues
No new toolkit issues found this pass; TK-001..TK-004 unchanged (analysis/toolkit-issues.md).
