# Round 3 — disassembly handoff

Status: **DONE — INCOMPLETE ROM** (5 unknown bytes remain; rounded 100.00% is not completion).

## Scope
- Input checkpoint pass-002 (maps/current.json = analysis/state.json sha256 c6d84b24...; 1,044 unknown).
- Consumed ready leads L-019, L-020 (analysis/redirect/r02-a/proposals.json). No gameplay or emulator experiments
  (all S01-S16 visited; no residual has a known runtime path). The 5 residual bytes left unknown as instructed.

## Verification of the leads (evidence-policy.md)
- `python3 analysis/redirect/r02-a/validate.py` on pass-002: output identical to r02-a/validate.txt — every Code proposal
  decodes exactly to its end, official opcodes only, last instruction RTS/JMP or falls into a known instruction start
  ($A67B, $DDAF, $DF9B), every branch/JSR/JMP target a known or proposed instruction start in the same/fixed bank.
- Hand review (r2dis.py) of every range and both neighbours: routine shapes and boundaries match the stated rationales
  (e.g. $A52E `jmp ($0016)` dispatch whose 16 table words $A560..$A581 are exactly the stub instruction starts;
  $A4C5-$A4DB operand tables bounded by RTS $A4C4 and code $A4DC; $9D43 bank-0 twin; $C9F8 shift-add multiply;
  $CAB5 bit test/set/clear trio; $E0EF-$E0F0 = signature bytes read by the cpx #$10 loop).
- Overlapped Data: rule-R singletons 0x0221E/0x0252C/0x02811 are page-crossing branch dummy reads
  (r02-a dummyreads.txt + dummytest-results.txt); 0x0221F-0x0222B was rule R "around clusters" with no read of its own and
  decodes as the tail of $A1FB-$A21B ending RTS before observed $A21C; rule-W 0x0254D+64 split (jmp ($0016) / table / stubs).
- No proposal rejected or narrowed. All new Code: Static provenance, confidence 0.85, views with effective bank context
  (bank 0 [0,-,14,15]; bank 1 [-,1,14,15]; bank 2 [2,-,14,15]; bank 12 [12,-,14,15]; banks 14/15 [-,-,14,15]);
  evidence text states "reachability not claimed, not asserted unused". Labels left to commentary (suggested names in
  proposals.json).

## Map edits (analysis/tools/r03/plan.py -> scratch/r03_plan.json; applied by analysis/tools/r02/apply.py)
- Removed Data ids 766 (0x0221E+1), 767 (0x0221F+13), 774 (0x0252C+1), 775 (0x0254D+64), 787 (0x02811+1).
- Added 24 Code regions (1,045 B) + 5 Data regions (42 B) + re-added Data 0x02550-0x0256F (32 B, $A540 jump table).
- Net: Unknown->Code 997 (L-019 734, L-020 263), Data->Code 48, Unknown->Data 42 (L-019 32, L-020 10).

## Checkpoint
- `python3 run.py checkpoint pass-003`: ok, byte_identical true; maps/pass-003.json, generated/pass-003,
  analysis/pass-003-verification.json; map sha256 703aba695f360c89b35583fabe94ca17e4d2f4bc4437837f76069504c92be4c4
  (= maps/current.json = analysis/state.json). map.validate ok.

## Counts (internal map)
ROM classification: 100.00% (262155 / 262160 bytes classified; 5 bytes unknown).
- Code 30,901 (Observed 20,357 / Static 10,544), Data 78,807, Text 13,377, Padding 7,998, Image 131,072, Unknown 5.
- Per bank: analysis/evidence/round-03/perbank.md (unknown b1 3, b15 2; all other banks and CHR 0).

## Remaining unknown (exact)
| Offset | Bank / CPU | Byte | Context |
|---|---|---|---|
| 0x02DC6 | b1 $ADB6 | RTS | after `lda #$64 / jmp $EDA3` |
| 0x032FA | b1 $B2EA | RTS | after `jmp $B4D7` |
| 0x03353 | b1 $B343 | RTS | after `jmp $B3C5` |
| 0x1E106 | b15 $E0F6 | NOP | after the new stop loop $E0F1-$E0F5 |
| 0x1E1C3 | b15 $E1B3 | NOP | between RTS $E1B2 and IRQ handler $E1B4 |

## Leads
- Consumed: L-019 verified (applied), L-020 verified (applied).
- Produced: L-021 (open, low) — the 5 residual bytes above; redirect re-test per coordinator note (RTS-trick address-1
  references, split hi/lo tables, script native-call operand encodings) and the r02-a rejection review.
- Unchanged: L-008 deferred (natural bite; no classification value).

## Commentary needs
- New Static code is unlabeled: bank-1 fish residues (suggested names in proposals.json), bank-0 $811A/$9D43,
  bank-2 $87A4, bank-12 $8A72, bank-14 $C085/$C9F8/$CAB5/$DDA3/$DF76, bank-15 $E0F1 stop loop; previous "unknown"
  notes at these addresses are now stale.

## Toolkit
No new toolkit issues found this pass; TK-001..TK-004 unchanged (analysis/toolkit-issues.md). Servers: this pass's
rpcd/serve stopped (an unrelated `run.py serve` from another workspace was left alone).
