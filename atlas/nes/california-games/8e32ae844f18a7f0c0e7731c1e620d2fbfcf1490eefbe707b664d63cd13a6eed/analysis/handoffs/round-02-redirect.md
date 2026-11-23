# Round 2 - redirect phase handoff (attempt r02-a1)

Status: FINISHED for this redirect pass. Recommendation: **(a) next disassembly cycle** with ready leads
L-R2R-01..04 (143 of 149 residual bytes). After that cycle the expected remainder is 6 bytes. Both of those
residuals are judged exhausted (see the rejection review), so the redirect after it should be terminal unless the
reviewer rejects parts of this batch.

## Scope and checkpoint
- Input: maps/round02-final.json = maps/current.json = analysis/state.json export, sha256 53e08b7ba423...
  (99.89%, 130939/131088; 149 B unknown in 18 gaps). Source commentary/source/game.asm 2ed461a9...
- ROM, map, state.json, source and binary are unchanged. analysis/state.json sha was recorded before the work
  (analysis/redirect/r02-a1/state.before.sha). Only emulator calls were made; there were no map edits.
- Research directory: analysis/redirect/r02-a1/ (NOTES.md with a per-gap decision table, tools/, evidence/,
  leads.before.json). r01-a1 is untouched.
- Inputs consumed: L-R2-01 (now `consumed`); L-R1-01 re-checked (still `blocked`, attempt appended).

## Ready leads (analysis/leads.json; exact ranges and rationale in analysis/redirect/r02-a1/evidence/proposals.json)
| Lead | Kind | Bytes | Ranges (CPU) | Structural evidence |
|---|---|---|---|---|
| L-R2R-01 | Code | 29 | b0 $8229-$822A, $8CED-$8CF5; b7 $D7B9-$D7BD, $F150-$F159, $F2BA-$F2BC | Each is an argument preamble feeding an established routine contract: UploadStatusRow A = row low byte; ClearStatusLineY Y, with the same shape as FootBagSummaryDone; QueueSfx Y slot stride $10/$14/**$18**/$1C. $F150 is a self-closed loop (its JMP targets its own 2nd instruction). $F2BA is the bank-0 link of the CallBank BIT chain (A9 00 2C, A9 01 2C ...). |
| L-R2R-02 | Code | 9 | b0 $8A85; b7 $DEBF; b7 $FFF7 (3 B each) | A lone JMP to an exact established entry (IdleLoop, SurfUpdate, ResetVector), at an instruction boundary after a terminator or non-code, with no data consumer. Weakest lead: if the reviewer applies the single-instruction caution strictly, leave the bytes unknown. |
| L-R2R-03 | Data (specific) | 18 | b0 $87CB-$87CF, $BEA3-$BEA7; b7 $EEB5-$EEB6, $F14A-$F14F | Consumer-based extents. FrameHandlerTable: max NmiState $1C, so the table ends at $87CF. BmxStart OAM copy (ldx #$1B from $BE8C) ends at $BEA7. $EEAF,y with CurrentPlayer 0..7 (capped by cmp #$07) ends at $EEB6. BinToBcdTable is 100 entries. |
| L-R2R-04 | Data (generic) | 87 | b2 $87DF-$87E6; b0 $9B46-$9B47; b7 $F072-$F079, $F883-$F8AF, $FFDF-$FFF6 | Non-code. b2 $87DF is a $00 run after a forced branch (the BRK vector is reset). $9B46 would be a discarded `ora` before a routine that overwrites A. $F072 has unofficial opcodes in every alignment. $F883 and $FFDF contain ROM writes (bank switch), unmapped $5A61/$6090, jsr $0000 and illegal opcodes, inside data blocks. |

Reachability: none of the Code is referenced; record it as Static with "no caller/selector known", not Observed
or dead.

Recipe for disassembly (all leads):
1. Run `python3 analysis/redirect/r02-a1/tools/proposals.py`. It is read-only and asserts every byte is still Unknown.
2. Independently re-check each cited consumer/contract line in game.asm (line numbers are in leads.json).
3. map.annotate kind/Static/rationale. Code view: prg_banks [0,1,14,15]. evidence/toolkit-decode.txt holds the
   configured-binary decode of every Code range.
4. **$F2BA special case:** map.remove region id 578 (0x1F2CD, 52 B, Observed, view $F2BD). Annotate
   [0x1F2CA, 5] Code Static view $F2BA (lda #$00 / bit $01A9), then re-annotate [0x1F2CF, 50] Code Observed view
   $F2BF with the original evidence. The decode alignment from $F2BF is unchanged. CallBank1 ($F2BD) becomes a
   BIT-operand constant like CallBank2..7. If the re-split is declined, leave 0x1F2CA-0x1F2CC unknown.
5. Run map.validate and the byte-identical rebuild. Then commentary for the new ranges, plus the corrections below.

Expected result if everything is accepted: 131082/131088 (99.995%). The remaining unknowns would be
0x18-0x1A and the TK-04 bytes.

## Findings (commentary-relevant)
- The FrameHandlerTable note at game.asm:1282 misread the $87CB gap. Entry $1C (LicenceFrame) is at $87CE/$87CF,
  so the table ends exactly at $87CF.
- PlayerNames record stride is **7** (6 name bytes + team byte; PlayerTeams = $03DA+6), not 6 as the symbol notes say.
  $EEAF-$EEB6 is a second copy of PlayerNameOffsets used by SkateFrame_Finish. Up to 8 players.
- $BE8C-$BEA7 is the BMX initial OAM image: sprite 0 at Y=$C8 is the split marker, then 6 sprites with tile $BC.
- $F150 fills $0600 (TerrainTableA) with 0..255 forever. Purpose unknown; it may be debug code.
- PlaySfx slot 6 ($D7B9) has no `sta SndTemp`. A caller would have to preset SndTemp. Its existence completes the
  $10..$1C slot series.
- S12 loop (compete restarted at HALF PIPE): explained statically. In EventIntroFrame, CurrentEvent = 6 plus any new
  button press jumps to ShowMainMenu, so random mashing skips FINAL RESULTS. Natural-route checks:
  evidence/natural-ending-run1-seed7.json (30000 frames mashed: stalled in skating, ending not reached) and
  run 2 (see the Gameplay section).

## Compact rejection review (residuals considered exhausted)
1. **b0 $8008-$800A `20 93 D8` (3 B), kept Unknown.**
   - Claim tested: code (jump-table slot 0 entry) vs generic data.
   - Decisive evidence: decode = jsr UpdateSound ($D893, a valid fixed-bank entry), in the position where banks
     1-6 hold jump-table entry 0. The return continuation $800B is the verified copyright text (Data;
     " (C)" = 20 28 43 = jsr $4328, unmapped).
   - Inline-argument interpretation: ruled out, because UpdateSound returns normally to code at other sites
     (e.g. $8A73 -> $8A76 lda $0C).
   - Caller: none. Both CallBank0 ($F2BA) and Y=$08 into bank 0 are uncalled.
   - What this rules out: a functioning routine. It does not rule out a vestigial or never-used entry. No data
     role or consumer exists either.
   - Strongest competitors: (a) a dead jump-table placeholder (code-shaped, broken continuation);
     (b) non-code header bytes.
   - Why insufficient: a Code region would end in a JSR flowing into Data, which fails the verifier's
     termination rule. Data would rest only on "not a working routine", and the policy says lack of a working
     path alone is not data. Code vs data stays materially ambiguous.
   - What would change it: a caller or selector for bank-0 entry 0, or a policy decision allowing a
     non-terminating dead entry.
2. **b0 $BFFE-$BFFF + b7 $C000 (3 B, TK-04).**
   - Claim: an observed `lda $C149,x` spanning physical 0x400E, 0x400F and 0x1C010.
   - Re-checked against the toolkit source (analysis/toolkit-issues.md TK-04 update). Code regions need views, and
     decode_region requires physically contiguous instruction bytes within the region. No prg_banks value maps
     $BFFF/$C000 to adjacent physical bytes.
   - Data was rejected: it contradicts the observed opcode fetch.
   - Not resolvable without a toolkit change; that change is not authorized.

## Deferred avenues
- Specific roles for nearby generic data, done cheaply:
  - Done: BmxInitOam, the $EEAF player offsets, the FrameHandlerTable extent and BinToBcdTable (notes in L-R2R-03).
  - Not pursued: the rest of the bank-7 $F69E-$FF68 data block. It has no consumer reference (xref found none), so
    its format would need speculative graphics decoding with no classification gain.
  - Not pursued: $9B3E table extent. Its Y domain depends on the HalfPipeAnimate Y paths, which include Y >= $0A.
    Generic data does not need it.
- S12/S14 natural ending: see Gameplay. Further mashing attempts are not planned because of diminishing value: the
  ending is already visited artificially (S14), its code is classified, and the remaining unknowns are not
  gameplay-dependent.
- No other promising avenue remains. All 18 gaps have a decision, and none of the remaining unknowns depends on
  runtime state.

## Gameplay
- Natural compete-all continuation from scratch/snap_r7.json (artificial=false, no RAM writes; tools/natural_end.py).
  - Run 1 (seed 7, a/b/left/right, 30000 frames): foot bag -> surfing -> skating, then stalled in skating.
  - Run 2 (seed 11, right-biased, 36000 frames): foot bag -> surfing (natural NmiState $1A SurfRideEnd at frame
    28958 and $18 SurfScoreFrame at 29540, which supports L-R2R-03) -> skating -> BMX -> flying disk, then stalled
    in flying disk (NmiState $0C) from frame 35360.
  - The ending was not reached naturally. S12 stays `attempted`, with the attempt and the static loop explanation
    recorded in analysis/scenarios.json. S14 (FINAL RESULTS, artificial) is unchanged.
- No further mashing is planned (diminishing value; see Deferred avenues).

## Toolkit
No new issues. TK-04 has an exact-blocker update. TK-01..05 are otherwise unchanged.

## Disk
About 3.0G free. The only new scratch files are scratch/r02_natural.log and r02_natural2.log (disposable). No snapshots or pngs
were produced. Evidence JSONs are in analysis/redirect/r02-a1/evidence/.
