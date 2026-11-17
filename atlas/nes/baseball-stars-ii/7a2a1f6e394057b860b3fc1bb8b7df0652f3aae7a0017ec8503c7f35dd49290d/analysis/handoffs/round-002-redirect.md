# Round 2 redirect handoff (attempt analysis/redirect/r2-redirect-a) - COMPLETE

Recommendation: **next: disassembly round 3**. One combined proposal batch is ready: L023 and L024, plus the L021 residue.

## Scope and checkpoint
- Input checkpoint: maps/pass-002.json, which equals maps/current.json and analysis/state.json (sha256 ebcf33141ae8...).
  - Classification is 99.56%, with 1,727 unknown bytes in 83 runs.
  - The round-002 commentary handoff (complete) was accepted as input.
- This redirect changed research and lead records only. ROM, map, state, source and the configured binary are untouched.
  - The emulator ran on a /tmp copy of the store, and no map.save was done.
- The r1-redirect-a tools (lib.py, flow contracts) were reused by reference.
- The exact unknown complement was refreshed: r2-redirect-a/unknown-raw.json, 83 runs, 1,727 bytes.
- All 83 runs were reviewed against their neighbours, linear decode, refscan.py operand references and the commentary
  inspection notes (commentary/notes/99_round2_unknown.py).

## Proposal (analysis/redirect/r2-redirect-a/proposals.json; reproduce with `python3 props.py`)
| | Unknown bytes |
|---|---|
| Proposed | **1,702 of 1,727** |
| Code/Static | 1,128 |
| Data/Static (incl. inline-parameter bytes) | 574 |
| Phase-1 Data bytes reclassified to Code | 23 |
| Residual unknown | **25** (6 runs) |

There are 116 items. Each item has an exact physical range, CPU range, bank, prg_banks context, evidence and an optional label.

- L023 (173 bytes, Data): table pieces whose consumer index domain bounds the extent:
  - SidePlayerRecordPtrs: side 0/1.
  - EditedLineupPtrs: team 12..17, six words.
  - PreGamePitcherListPtrs.
  - PositionLetters and PositionLettersB9: positions 1..10.
  - The bank 9 adc operand at $B31B.
  - The $B852/$B8C6 records.
  - The $B005 VRAM-row words.
  - StadiumAttrVram, StadiumAttrPatterns (8 words plus the four 3-byte patterns they address), DigitTiles $AE1E and
    BlankNameField.
  - The bank 24 $8F07/$95A9/$9795/$9CBA tables.
  - TeamBlockDstPtrs $CF53-$CF5B.
- L024 (1,529 bytes):
  - 35 code groups (C01-C35). Each one either:
    - starts after a terminator or data boundary, with a static closure (pass-001 contracts) that covers the island
      exactly, uses official opcodes only, has 0 bad paths and 0 misaligned overlaps (codecheck.json, aligncheck.py); or
    - is a byte-identical copy of classified code.
  - Generic or role data after inspection: series and palette continuations, ASCII text, tile groups, and bytes for
    which no decode ends at the next code boundary.

### Key new facts (corrected premises over round-1 and pass-002 rejections)
1. **Bank 25 $A040 (169 bytes).** It was excluded in r1 because "the start PHP follows table $A02B-$A03F of unproven
   extent". That premise was wrong.
   - $A033-$A042 is `LDA #$0C / LDY #$08 / STA $002D / JSR $C32D` plus the 6-byte FarCall Y=8 inline record
     `12 0A 7C 08 85 20`. This is byte-identical to classified code at bank 25 $A49F and bank 24 $9807.
   - So the routine starts at $A033, and the phase-1 Data region 0x03203B+21 holds only the 4-word table $A02B-$A032
     ($6200/$6400/$6320/$6520, read by $A090).
   - Proposed:
     - Code 0x032043-0x03204C (reclassified).
     - Inline Data 0x03204D-0x032052.
     - Code/inline from 0x032053 to 0x0320F8.
2. **Bank 25 $B82B (97 bytes).** The routine runs past its unknown run into phase-1 Data 0x03389C+22.
   - The path is JSR $C6D6 with an inline word, then `JSR $C6FC $2102 / LDA #$0E / STA $0014 / JSR $C746 / JMP $C3CD`.
   - It is followed by the 7-byte table $B89B-$B8A1, read by `ldy $B89B,x`.
   - Proposed reclassification to Code: 0x03389C-0x03389D and 0x0338A0-0x0338AA. Inline $B88E-$B88F and the table
     stay Data.
3. **Bank 25 $A120/$A141.**
   - `lda $A11D,y` with y = team*2 and team 12..17 addresses exactly the six words $A135-$A140.
   - In $A141, v = $31..$35 indexes table $A1C2 at ((v&7)-1)*4 + 0..3, which ends exactly at $A1D5. The 4-byte table
     $A1D6-$A1D9 ends exactly at classified code $A1DA. This gives an exact extent proof for the run at $A1D4.
4. **Bank 30 $D199-$D1AC** is an exact 20-byte copy of Observed bank 25 $B8C9-$B8DC.
5. **Bank 30 $C9A8** is `LDA $FED0 / STA $35 / LDA $FED1 / STA $36 / JMP ($0035)`, a direct dispatch through FarCall
   entry 14 ($CE52).
6. **Mutually consistent island pairs**: b3 $BAE2 calls $B0B3; b15 $AAAB/$AAB6 call $AB1D; b15 $ADEE calls $AE28;
   b10 $8F10 calls $8F5D (with helper $8F4F between them); b25 $B82B calls $B8A2/$B8AC.

Weak items (the reviewer may keep them unknown):
- C18: b24 $9F6E BPL/LDA #0/RTS, whose entry depends on the caller's flags.
- C02: a single CLC entry. It mirrors the existing SEC/JMP pair.
- C31, C32, C33: 2-5 byte fall-through entries.
- D03: b1 $BA1F. The only decode is a JSR into the RAM mirror.

## Residual unknown (25 bytes; dispositions)
| Range | Bytes | Why it stays unknown |
|---|---|---|
| b2 0x0040EA-0x0040EB ($80DA) | 2 | Lone RTS-valued bytes |
| b2 0x005B58 ($9B48) | 1 | Lone RTS-valued byte |
| b9 0x012175 ($A165) | 1 | Lone RTS-valued byte |
| b2 0x005998-0x00599B ($9988) | 4 | `LDA #$80 / LDA #$00` before code that reloads A |
| b30 0x03C0D0-0x03C0D4 ($C0C0) | 5 | `LDA $22 / STA $22 / RTS`, which does nothing |
| b31 0x03F6D2-0x03F6DD ($F6C2) | 12 | Add-18 fragment with inconsistent carry use (`ADC $0D` right after `STA $0C`). Its duplicate table is proposed as D63 |

- The lone RTS bytes are unreferenced return opcodes; the policy says those lack sufficient structure on their own.
- None of these runs has a reference. For all of them, code versus data remains materially ambiguous.

## Gameplay (analysis/scenarios.json S23-S25; frames in r2-redirect-a/frames/)
- S23: TRADE PLAYER completed between two edited teams. The second team came from MAKE TEAM through an artificial
  $0552=4 menu cursor; the trade itself used natural menu input with a port-2 controller.
- S24: RELEASE, then ACQUIRE VETERAN ($8,000 contract, bonus allocation).
- S25: a 6-team / 24-game league: set-up, 6x6 standings and one simulated game.
- Traced aggregates: 11,221 instruction starts (r2-observed.json). There were **0 starts in any unknown run or
  proposal item**, so all structural claims stay Static and their reachability is unknown.
- New snapshots: scratch/snaps/mainmenu_two_teams.json and league6_after_game1.json.
- The league-champion ending was already visited (S06).
- Completing the 6-team league was deferred: it needs 23 more simulated games and has no structural target.

## Leads (analysis/leads.json, pass redirect-002)
- Produced: L024 (ready).
- Updated:
  - L023: ready.
  - L021: ready, superseded by L024; it now records the 25-byte residue.
  - L006: deferred. The only remaining item is 6-team league completion.

## Deferred work
- The 25 residual bytes. The missing fact is any reference, or a structure that discriminates code from data.
  - Static review is done.
  - Traced menus (trade, veteran, league) did not fetch them.
  - Prospect is diminishing: the runs are 1-12 byte fragments with no consumer.
- 6-team league completion: low prospect. The champion screen is already visited and the shared engine is already traced.

## Artifacts
r2-redirect-a/:
- Analysis: view.py, refscan.py/refs.json, unknown-raw.json, codecheck.py/codecheck.json, aligncheck.py, props.py,
  proposals.json.
- Emulator: rdaemon.py/rr.py, make2.py, trade.py, step.py, cov.py, r2-observed.json.
- Snapshots of the inputs: leads.before.json, scenarios.before.json.
- frames/.

The scratch directory is cleaned (scratch/r2 removed). No new toolkit issues; the TI-001 traced-frame slowdown was seen again.
