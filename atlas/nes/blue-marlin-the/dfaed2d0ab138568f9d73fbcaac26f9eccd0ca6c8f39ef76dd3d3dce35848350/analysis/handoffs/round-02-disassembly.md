# Round 2 — disassembly handoff

Status: **DONE — incomplete ROM (1,044 bytes unknown).**

## Scope
One disassembly pass on checkpoint pass-001 (17,738 unknown): consume the ready redirect batch (L-004, L-005, L-009,
L-010, L-012, L-013, L-015, L-016) after re-checking each against evidence-policy.md, re-audit round-1 rule-R data,
bounded inspection of every remaining gap, one new gameplay milestone (S15 sea events). Tools: `analysis/tools/r02/`
(common.py, plan.py, plan_b.py, apply.py, raudit.py, rcontam.py, bankaudit.py, gapdump.py, gapcons.py, period4.py,
ptext.py, maptext.py, fix_maptext.py, fix_l018.py, probe_s15.py, newobs.py, counts.py, view.py). Server: one
`run.py serve` via analysis/tools/rpcd.py (stopped).

## Checkpoint and verification
- **pass-002**: maps/pass-002.json, generated/pass-002/, analysis/pass-002-verification.json —
  `ok: true, byte_identical: true`, map sha256 c6d84b24d5a969b1353e5443aca4b1ca2be098aa0ce21a7019dddadf53f5d425
  (identical to maps/current.json). map.validate ok; internal store analysis/state.json saved.

## Classification (internal map, unique bytes)
ROM classification: 99.60% (261116 / 262160 bytes classified; 1044 bytes unknown).

| Kind | Bytes |
|---|---|
| Code | 29,856 (Observed 20,357; Static 9,499) |
| Data | 78,813 |
| Text | 13,377 |
| Padding | 7,998 |
| Image (CHR) | 131,072 |
| Unknown | 1,044 |

Per bank unknown: 0: 72, 1: 769, 2: 9, 12: 6, 13: 8, 14: 171, 15: 9, all others 0 (full table
analysis/evidence/round-02/perbank.md; residual ranges analysis/evidence/round-02/residual-unknown.txt).
Execution coverage is separate: round-2 trace 5,208 ROM opcode addresses (all on pass-002 instruction starts).

## Leads consumed (analysis/leads.json attempts "disassembly-r2")
| Lead | Outcome | Notes |
|---|---|---|
| L-013 | accepted | static_rd_r01a ⊇ all pass-001 Code, same instruction starts except L-009 region; overlaps Unknown 4,918 + Data 286 (R 102, W 184), no Text/Padding. 40 Data regions trimmed (remainders kept with note); $ADB6 lone RTS (contaminated-cluster neighbour, no read/ref) left Unknown. Rule-R re-audit: no further contamination signature (29 uniform-read chains = linear sprite/tile stream copies). |
| L-004 | accepted | script $38/$04/$05 operands as roots (ScrCmd_CallTest = JMP (ScriptArgC6)); plus $ED52 (newly found $38 operand in bank-12 script data) and sibling $EE09 as 3-byte entries whose branch lands on observed instruction starts. |
| L-009 | accepted | 0x03A3A-0x03A5C one routine; misaligned static region 0x03A3E removed. |
| L-010 | accepted | bank-0 wrappers + callees Static code; **then observed** in S15 ($8BF5/$8C33/$8C71 executed). |
| L-005 | accepted | script/switch -> Data, PrintText strings -> Text over Unknown only; re-splitting existing Data/Text deferred. |
| L-012 | accepted | 0x1E5EC-0x1E60D Data, label FightEventScript. |
| L-015 | accepted | consumers re-located; $F75E string = generic Data (no consumer). |
| L-016 | accepted | write audit re-run (analysis/evidence/round-02/l016-bank-select-audit.txt); 7,389 B generic Data. |
| L-018 | produced + applied | 71 banks-2/3 rule-T Text regions (871 B) are note streams -> Data (role correction). |

## Batch B residual inspection (static, bounded)
Data with exact consumers/record formats (banks 0/1/12-15 tables, banks 6/7 metasprite and pointer data, bank 4
graphics streams, bank 2 sound data after the driver's last code byte $8A14, footer "BLUE MARINE Ver1" + 10 bytes);
Text: bank-0 window-text control bytes checked with the **MapText** grammar (MapText_Control $8235; corrected mid-pass
from an initial PrintText-grammar check — all 42 regions re-tokenized, evidence rewritten), bank-4/12/13/15 string
terminators, status/name strings with pointer tables ($8EE6, $9ED7).

## Scenarios (analysis/scenarios.json)
S15 now visited (artificial): sea snapshot + SeaEventFlag $05C4 = 1/2/3 -> Nav_RunEvent -> bank-0 event wrappers;
event 3 reloads the area. Return-to-port menu itself not reached. Frames analysis/evidence/round-02/frames/,
coverage analysis/evidence/round-02/observed-r02.json.gz. Further play gave no new unknown-range execution; stopped.

## Leads produced
- **L-017** (open): unreferenced code-shaped residues — bank 1 $A1B9, $A3ED-$A5DD (note: genuine data reads at
  $A51C/$A3DC/$A3EC inside it), $A66C, $A7D6, $A802, $A8B1, $AEFF, $B424, lone RTS $ADB6/$B2EA/$B343; bank 0 $811A,
  $9D43; bank 14 $C085, $C9F8, $CAB5, $DDA3, $DF76; bank 15 $E0EF hang stub, $E1B3; bank 2 $87A4; bank 12 $8A72;
  bank 13 $A69B. Recipe: split lo/hi / RTS-dispatch (target-1) tables, unreached bank-12/13 script blocks (around
  $9380) for $38 operands, fights with other species.
- L-018 (verified, applied).

## Unresolved / deferred
- The 1,044 unknown bytes above (code vs data materially ambiguous: decodes as complete routines but no reference).
- Optional: re-split existing generic script Data/Text into labelled script entries (L-005 optional part) — commentary.
- Return-to-port menu (FishOnLine bit7 / $049E) and natural bite (L-008) not reached; not needed for classification now.
- Round-1 static conflicts at 0x031F9 (bank 1 $B1E9) and 0x08013 (bank 4) remain recorded, unclassified alternatives.

## Toolkit summary (analysis/toolkit-issues.md)
No new toolkit issues found in this pass. Existing: TK-001 confirmed bug (`emulator.input` needs `buttons`), TK-002
missing feature (MMC3 discovery stops at JSR; local static_rd used), TK-003 missing feature (tracing ~2.5 fps),
TK-004 missing feature (trace.clear wipes coverage). No blockers.
